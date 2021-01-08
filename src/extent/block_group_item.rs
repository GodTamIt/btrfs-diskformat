use {
    byteorder::LE,
    enumflags2::BitFlags,
    static_assertions::const_assert_eq,
    strum::EnumIter,
    zerocopy::{AsBytes, FromBytes, Unaligned, U64},
};

/// Defines the location, properties, and usage of a block group.
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct BlockGroupItem {
    /// The space used, in bytes, in this block group.
    pub used: U64<LE>,

    /// The object ID of the chunk backing this block group.
    pub chunk_objectid: U64<LE>,

    /// Flags indicating allocation type and replication policy.
    pub flags: U64<LE>,
}
const_assert_eq!(std::mem::size_of::<BlockGroupItem>(), 24);

/// The type of storage a block group allows.
///
/// [Data](AllocationType::Data) and [Metadata](AllocationType::Metadata) chunks may be mixed within
/// a [BlockGroupItem], as indicated by its [flags](BlockGroupItem::flags). However,
/// [System](AllocationType::System) chunks cannot be mixed.
#[derive(BitFlags, Copy, Clone, Debug, Hash, PartialEq)]
pub enum AllocationType {
    Data = 0x1,
    System = 0x2,
    Metadata = 0x4,
}

/// The replication policy a [BlockGroup] implements. Only one policy may be set for a given group.
#[derive(BitFlags, Copy, Clone, Debug, Hash, PartialEq, EnumIter)]
#[repr(u64)]
pub enum ReplicationPolicy {
    RAID0 = 0x8,
    RAID1 = 0x10,
    DUP = 0x20,
    RAID10 = 0x40,
    RAID5 = 0x80,
    RAID6 = 0x100,
    RAID1C3 = 0x200,
    RAID1C4 = 0x400,
}
