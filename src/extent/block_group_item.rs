use {
    bitflags::bitflags,
    byteorder::LE,
    static_assertions::const_assert_eq,
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

bitflags! {
    /// The type of storage a block group allows.
    ///
    /// [DATA](AllocationType::DATA) and [METADATA](AllocationType::METADATA) chunks may be mixed within
    /// a [BlockGroupItem], as indicated by its [flags](BlockGroupItem::flags). However,
    /// [SYSTEM](AllocationType::SYSTEM) chunks cannot be mixed.
    pub struct AllocationType: u64 {
        const DATA = 0x1;
        const SYSTEM = 0x2;
        const METADATA = 0x4;
    }
}

bitflags! {
    /// The replication policy a [BlockGroup] implements. Only one policy may be set for a given group.
    pub struct ReplicationPolicy: u64 {
        const RAID0 = 0x8;
        const RAID1 = 0x10;
        const DUP = 0x20;
        const RAID10 = 0x40;
        const RAID5 = 0x80;
        const RAID6 = 0x100;
        const RAID1C3 = 0x200;
        const RAID1C4 = 0x400;
    }
}
