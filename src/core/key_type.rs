use num_enum::{IntoPrimitive, TryFromPrimitive};
use static_assertions::const_assert_eq;
use strum::EnumIter;
use zerocopy_derive::*;

/// Recognized item types in a tree key.
///
/// [`Key::key_type`] remains a raw byte so unknown future values can be preserved.
/// Recognition of a type does not imply support for its item payload.
/// See the [on-disk format] for item descriptions.
///
/// [`Key::key_type`]: crate::Key::key_type
/// [on-disk format]: https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html
#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    PartialEq,
    EnumIter,
    IntoPrimitive,
    TryFromPrimitive,
    TryFromBytes,
    IntoBytes,
    KnownLayout,
    Immutable,
)]
#[repr(u8)]
pub enum KeyType {
    InodeItem = 1,
    InodeRef = 12,
    InodeExtRef = 13,
    XattrItem = 24,
    VerityDesc = 36,
    VerityMerkle = 37,
    OrphanItem = 48,
    DirLogItem = 60,
    DirLogIndex = 72,
    DirItem = 84,
    DirIndex = 96,
    ExtentData = 108,
    CsumItem = 120,
    ExtentCsum = 128,
    RootItem = 132,
    RootBackref = 144,
    RootRef = 156,
    ExtentItem = 168,
    MetadataItem = 169,
    ExtentOwnerRef = 172,
    TreeBlockRef = 176,
    ExtentDataRef = 178,
    ExtentRefV0 = 180,
    SharedBlockRef = 182,
    SharedDataRef = 184,
    BlockGroupItem = 192,
    FreeSpaceInfo = 198,
    FreeSpaceExtent = 199,
    FreeSpaceBitmap = 200,
    DevExtent = 204,
    DevItem = 216,
    ChunkItem = 228,
    /// Experimental RAID stripe mapping.
    RaidStripe = 230,
    /// Experimental identity remapping.
    IdentityRemap = 234,
    /// Experimental remapping.
    Remap = 235,
    /// Experimental remapping back reference.
    RemapBackref = 236,
    QgroupStatus = 240,
    QgroupInfo = 242,
    QgroupLimit = 244,
    QgroupRelation = 246,
    TemporaryItem = 248,
    PersistentItem = 249,
    DevReplace = 250,
    UuidSubvol = 251,
    UuidReceivedSubvol = 252,
    StringItem = 253,
}
const_assert_eq!(core::mem::size_of::<KeyType>(), 1);
