use {
    crate::Time,
    bitflags::bitflags,
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U32, U64},
};

/// Contains traditional inode data and attributes.
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct InodeItem {
    // FIXME: add documentation!
    pub generation: U64<LE>,

    // FIXME: add documentation!
    pub transid: U64<LE>,

    /// The size of a file, in bytes.
    pub size: U64<LE>,

    /// The size allocated to the file, in bytes.
    ///
    /// This is equal to the sum of all of the extent data for the inode.
    /// This is 0 for directories.
    pub nbytes: U64<LE>,

    /// This contains the byte offset of a block group when structure is a free space inode.
    ///
    /// This value is unused for normal inodes.
    pub block_group: U64<LE>,

    /// Count of inode references for the inode.
    ///
    /// When used outside of a file tree, this value is 1.
    pub nlink: U32<LE>,

    /// The user ID of the owner in Unix.
    pub uid: U32<LE>,

    /// The group ID of the group owner in Unix.
    pub gid: U32<LE>,

    /// The Unix protection mode.
    pub mode: U32<LE>,

    /// The device identifier (if a special file).
    pub rdev: U64<LE>,

    /// Flags for the inode. See [InodeFlags] for values.
    pub flags: U64<LE>,

    /// A sequence number used for compatibility with NFS.
    ///
    /// This value is initialized to 0 and incremented each time [mtime](InodeItem::mtime) is
    /// updated.
    pub sequence: U64<LE>,

    pub _unused: [u64; 4],

    /// Timestamp of the last access to the inode.
    pub atime: Time,

    /// Timestamp of the last change to the inode's properties.
    pub ctime: Time,

    /// Timestamp of the last change to the inode's contents.
    pub mtime: Time,

    /// Timestamp of the creation of the inode.
    pub otime: Time,
}
const_assert_eq!(std::mem::size_of::<InodeItem>(), 160);

bitflags! {
    pub struct InodeFlags: u64 {
        /// Do not perform checksum operations.
        const NO_DATA_SUM = 0x1;

        /// Do not perform copy-on-write for data extents when the reference count is 1.
        const NO_DATA_COW = 0x2;

        /// The inode is read-only, regardless of permissions or ownership.
        const READ_ONLY = 0x4;

        /// Do not perform compression.
        const NO_COMPRESS = 0x8;

        /// Denotes preallocated extents are present. Hints that filesystem should avoid CoWing
        /// those extents.
        const PREALLOC = 0x10;

        /// Operations on this inode should be performed synchronously.
        const SYNC = 0x20;

        /// The inode is read-only; regardless of permissions or ownership.
        const IMMUTABLE = 0x40;

        /// The inode is append-only.
        const APPEND = 0x80;

        /// Do not consider the inode for dumping when using the Unix program `dump`.
        const NO_DUMP = 0x100;

        /// Do not update [atime](InodeItem::atime).
        const NO_ATIME = 0x200;

        /// Operations on directory should be performed synchronously.
        const DIR_SYNC = 0x400;

        /// Perform compression for the inode.
        const COMPRESS = 0x800;
    }
}
