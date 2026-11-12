use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
};

use btrfs_diskformat::{SuperBlock, constants::PRIMARY_SUPERBLOCK_ADDR};
use tempfile::TempDir;
use zerocopy::TryFromBytes;

pub struct DiskSet {
    directory: Option<TempDir>,
    images: Vec<PathBuf>,
    loops: Vec<PathBuf>,
    mountpoint: PathBuf,
    mounted: bool,
    registered: bool,
    attachment_uncertain: bool,
}

impl DiskSet {
    pub fn format(disks: usize, mkfs_args: &[&str]) -> Self {
        assert!(disks > 0, "a filesystem needs at least one image");
        let uid = run(Command::new("id").arg("-u"));
        assert_eq!(
            uid.stdout,
            b"0\n",
            "mount tests require effective uid 0; {}",
            diagnostic(Command::new("id").arg("-u"), &uid)
        );
        let directory = tempfile::Builder::new()
            .prefix("btrfs-mount-")
            .tempdir()
            .expect("create private test directory");
        let mountpoint = directory.path().join("mount");
        let mut set = Self {
            directory: Some(directory),
            images: Vec::new(),
            loops: Vec::new(),
            mountpoint,
            mounted: false,
            registered: false,
            attachment_uncertain: false,
        };
        fs::create_dir(&set.mountpoint).expect("create private mountpoint");
        for index in 0..disks {
            let image = set.directory.as_ref().unwrap().path().join(format!("disk-{index}.img"));
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&image)
                .unwrap_or_else(|error| panic!("create {image:?}: {error}"))
                .set_len(512 * 1024 * 1024)
                .unwrap_or_else(|error| panic!("size {image:?}: {error}"));
            set.images.push(image);
        }
        run(Command::new("mkfs.btrfs").args(["-f", "-q"]).args(mkfs_args).args(&set.images));
        set
    }

    pub fn mount(&mut self, options: &str) {
        assert!(!self.mounted, "filesystem is already mounted");
        if self.loops.is_empty() {
            for image in &self.images {
                // A failed attachment can still have created a loop device.
                self.attachment_uncertain = true;
                let mut command = Command::new("losetup");
                command.args(["--find", "--show"]).arg(image);
                let output = run(&mut command);
                let device = String::from_utf8_lossy(&output.stdout);
                let device = device.trim();
                assert!(
                    device.strip_prefix("/dev/loop").is_some_and(|number| {
                        !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
                    }),
                    "invalid loop device; {}",
                    diagnostic(&command, &output)
                );
                self.loops.push(PathBuf::from(device));
                self.attachment_uncertain = false;
            }
        }
        let mut options = options.to_owned();
        for device in &self.loops {
            if !options.is_empty() {
                options.push(',');
            }
            options.push_str("device=");
            options.push_str(device.to_str().unwrap());
        }
        // Even a failing mount command may have registered devices or mounted.
        self.registered = true;
        self.mounted = true;
        run(Command::new("mount")
            .args(["-t", "btrfs", "-o"])
            .arg(options)
            .arg(&self.loops[0])
            .arg(&self.mountpoint));
    }

    pub fn unmount(&mut self) {
        assert!(self.mounted, "filesystem is not mounted");
        run(Command::new("umount").arg(&self.mountpoint));
        self.mounted = false;
        self.forget().unwrap_or_else(|error| panic!("{error}"));
    }

    pub fn mountpoint(&self) -> &Path {
        &self.mountpoint
    }

    pub fn superblocks(&self) -> Vec<SuperBlock> {
        assert!(!self.mounted, "superblocks must be read while unmounted");
        self.images
            .iter()
            .map(|image| {
                let mut file =
                    File::open(image).unwrap_or_else(|error| panic!("open {image:?}: {error}"));
                file.seek(SeekFrom::Start(PRIMARY_SUPERBLOCK_ADDR))
                    .unwrap_or_else(|error| panic!("seek {image:?}: {error}"));
                let mut bytes = [0; 4096];
                file.read_exact(&mut bytes)
                    .unwrap_or_else(|error| panic!("read superblock from {image:?}: {error}"));
                SuperBlock::try_read_from_bytes(&bytes)
                    .unwrap_or_else(|_| panic!("invalid superblock layout in {image:?}"))
            })
            .collect()
    }

    pub fn tune(&mut self, args: &[&str]) {
        assert!(!self.mounted, "btrfstune requires an unmounted filesystem");
        run(Command::new("btrfstune").args(args).arg(&self.images[0]));
    }

    pub fn finish(mut self) {
        let errors = self.cleanup();
        assert!(errors.is_empty(), "cleanup failed:\n{}", errors.join("\n"));
    }

    fn forget(&mut self) -> Result<(), String> {
        if self.registered {
            // Never omit the explicit paths: that would forget unrelated devices.
            checked(Command::new("btrfs").args(["device", "scan", "--forget"]).args(&self.loops))?;
            self.registered = false;
        }
        Ok(())
    }

    fn cleanup(&mut self) -> Vec<String> {
        if self.directory.is_none() {
            return Vec::new();
        }
        let mut errors = Vec::new();
        if self.mounted {
            match checked(Command::new("umount").arg(&self.mountpoint)) {
                Ok(_) => self.mounted = false,
                Err(error) => errors.push(error),
            }
        }
        if !self.mounted
            && let Err(error) = self.forget()
        {
            errors.push(error);
        }
        let devices = std::mem::take(&mut self.loops);
        for device in devices.iter().rev() {
            if let Err(error) = checked(Command::new("losetup").arg("--detach").arg(device)) {
                errors.push(error);
            }
        }
        if self.attachment_uncertain {
            errors.push(format!(
                "attachment outcome unknown; inspect losetup --associated for each image: {:?}",
                self.images
            ));
        }
        if let Some(directory) = self.directory.take() {
            if errors.is_empty() {
                let path = directory.path().to_owned();
                if let Err(error) = directory.close() {
                    errors.push(format!("remove {path:?}: {error}"));
                }
            } else {
                // Never let TempDir recursively remove a potentially live mount.
                let path = directory.keep();
                errors.push(format!(
                    "retained {path:?}; inspect mountpoint {:?} and unmount it if active; \
                     recorded loops: {devices:?}; verify their backing-file associations before \
                     manual forget/detach and directory removal; detached loops may remain \
                     pending autoclear",
                    self.mountpoint
                ));
            }
        }
        errors
    }
}

impl Drop for DiskSet {
    fn drop(&mut self) {
        for error in self.cleanup() {
            // Unlike eprintln!, a broken stderr must not panic during unwinding.
            let _ = writeln!(io::stderr().lock(), "DiskSet cleanup: {error}");
        }
    }
}

fn diagnostic(command: &Command, output: &Output) -> String {
    format!(
        "command: {command:?}\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn checked(command: &mut Command) -> Result<Output, String> {
    let output = command.output().map_err(|error| {
        format!(
            "command: {command:?}\nfailed to execute: {error}\nstatus/stdout/stderr: unavailable"
        )
    })?;
    if output.status.success() { Ok(output) } else { Err(diagnostic(command, &output)) }
}

fn run(command: &mut Command) -> Output {
    checked(command).unwrap_or_else(|error| panic!("{error}"))
}
