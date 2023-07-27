use {
    num_enum::{IntoPrimitive, TryFromPrimitive},
    strum::{EnumIter, IntoEnumIterator},
};

#[derive(
    Copy, Clone, Debug, Hash, PartialEq, PartialOrd, EnumIter, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u64)]
pub enum ReservedObjectId {
    /// The object ID for device stats in the device tree.
    DevStats = 0,

    /// The object ID that refers to the `ROOT_TREE` itself.
    RootTree = 1,

    /// The object ID that refers to the `EXTENT_TREE`.
    ExtentTree = 2,

    /// The object ID that refers to the root of the `CHUNK_TREE`.
    ChunkTree = 3,

    /// The object ID that refers to the root of the `DEV_TREE`.
    DevTree = 4,

    /// The object ID that refers to the global `FS_TREE` root.
    FsTree = 5,

    /// The object ID that refers to the directory within the root tree. If it exists, it will have
    /// the usual items used to implement a directory associated with it. There will only be a
    /// single entry called default that points to a key to be used as the root directory on the
    /// file system instead of the `FS_TREE`.
    RootTreeDirectory = 6,

    /// The object ID that refers to the `CSUM_TREE`.
    ChecksumTree = 7,

    /// The object ID that refers to the `QUOTA_TREE`.
    QuotaTree = 8,

    /// The object ID that refers to the `UUID_TREE`.
    UuidTree = 9,

    /// The object ID that refers to the `FREE_SPACE_TREE`.
    FreeSpaceTree = 10,

    /// The object ID for where balance parameters are written in the root tree.
    Balance = -4i64 as u64,

    /// The object ID used for orphan root tracking.
    Orphan = -5i64 as u64,

    /// The object ID that refers to the `TREE_LOG` tree.
    TreeLogTree = -7i64 as u64,

    /// The object ID that refers to the `TREE_RELOC` tree.
    TreeRelocationTree = -8i64 as u64,

    /// The object ID that refers to the `DATA_RELOC` tree.
    DataRelocationTree = -9i64 as u64,

    /// The object ID for all extent checksums.
    ExtentChecksum = -10i64 as u64,

    /// The object ID assigned to the inode used to store the free space cache.
    FreeSpace = -11i64 as u64,

    /// The object ID assigned to the inode used to store the free inode cache.
    FreeInode = -12i64 as u64,

    /// A dummy object ID representing multiple object IDs.
    MultipleObjectIds = -255i64 as u64,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ObjectId(u64);

impl ObjectId {
    pub const FIRST_FREE: Self = Self(256);
    pub const LAST_FREE: Self = Self(-256i64 as u64);

    pub fn is_valid_reserved(&self) -> bool {
        for reserved_id in ReservedObjectId::iter() {
            if self.0 == reserved_id.into() {
                return true;
            }
        }

        false
    }

    pub fn is_in_free_range(&self) -> bool {
        self >= &Self::FIRST_FREE && self <= &Self::LAST_FREE
    }
}

impl From<u64> for ObjectId {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<ObjectId> for u64 {
    fn from(id: ObjectId) -> Self {
        id.0
    }
}
