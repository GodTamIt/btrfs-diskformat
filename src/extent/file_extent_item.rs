use num_enum::{IntoPrimitive, TryFromPrimitive};
use static_assertions::const_assert_eq;
use strum::EnumIter;
use zerocopy::little_endian::{U16 as U16LE, U64 as U64LE};
use zerocopy_derive::*;

/// The common prefix of inline, regular, and preallocated file extents.
///
/// Inline payload bytes follow this prefix immediately. For regular and
/// preallocated extents, the remaining fields are described by [`FileExtentItem`].
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#extent-data-6c>
#[derive(Copy, Clone, Debug, IntoBytes, TryFromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct FileExtentItemHeader {
    pub generation: U64LE,

    /// An upper bound on the decoded extent size, in bytes.
    pub ram_bytes: U64LE,

    /// The compression encoding.
    pub compression: CompressionType,

    /// The encryption encoding (0 = none).
    pub encryption: u8,

    /// Other encoding (0 = none).
    pub other_encoding: U16LE,

    pub extent_type: FileExtentType,
}
const_assert_eq!(core::mem::size_of::<FileExtentItemHeader>(), 21);

/// A regular or preallocated file extent, including the common prefix.
///
/// This fixed layout is not for inline extents: their payload follows
/// [`FileExtentItemHeader`] instead of the disk extent fields below.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#extent-data-6c>
#[derive(Copy, Clone, Debug, IntoBytes, TryFromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct FileExtentItem {
    pub generation: U64LE,

    /// An upper bound on the decoded extent size, in bytes.
    pub ram_bytes: U64LE,

    /// The compression encoding.
    pub compression: CompressionType,

    /// The encryption encoding (0 = none).
    pub encryption: u8,

    /// Other encoding (0 = none).
    pub other_encoding: U16LE,

    /// Must be regular or preallocated for this layout.
    pub extent_type: FileExtentType,

    /// The logical address of the extent; zero denotes a hole.
    pub disk_bytenr: U64LE,

    /// The size of the encoded extent, in bytes.
    pub disk_num_bytes: U64LE,

    /// The offset within the decoded extent.
    pub offset: U64LE,

    /// The number of bytes represented in the file.
    pub num_bytes: U64LE,
}
const_assert_eq!(core::mem::size_of::<FileExtentItem>(), 53);

/// The compression encodings used by file extents.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#extent-data-6c>
/// * <https://btrfs.readthedocs.io/en/latest/Compression.html>
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
pub enum CompressionType {
    None = 0,
    Zlib = 1,
    Lzo = 2,
    Zstd = 3,
}
const_assert_eq!(core::mem::size_of::<CompressionType>(), 1);

/// The file extent types defined by the on-disk format.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/dev/On-disk-format.html#extent-data-6c>
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
pub enum FileExtentType {
    Inline = 0,
    Regular = 1,
    Prealloc = 2,
}
const_assert_eq!(core::mem::size_of::<FileExtentType>(), 1);
