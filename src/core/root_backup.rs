use {
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U64},
};

#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct RootBackup {
    pub tree_root: U64<LE>,
    pub tree_root_gen: U64<LE>,

    pub chunk_root: U64<LE>,
    pub chunk_root_gen: U64<LE>,

    pub extent_root: U64<LE>,
    pub extent_root_gen: U64<LE>,

    pub fs_root: U64<LE>,
    pub fs_root_gen: U64<LE>,

    pub dev_root: U64<LE>,
    pub dev_root_gen: U64<LE>,

    pub csum_root: U64<LE>,
    pub csum_root_gen: U64<LE>,

    pub total_bytes: U64<LE>,
    pub bytes_used: U64<LE>,

    pub num_devices: U64<LE>,

    /// Reserved for future use.
    pub _unused_u64s: [U64<LE>; 4],

    pub tree_root_level: u8,

    pub chunk_root_level: u8,

    pub extent_root_level: u8,

    pub fs_root_level: u8,

    pub dev_root_level: u8,

    pub csum_root_level: u8,

    /// Reserved for future use.
    pub _unused_u8s: [u8; 10],
}
const_assert_eq!(std::mem::size_of::<RootBackup>(), 168);
