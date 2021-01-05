use {
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, I64, U32},
};

/// The layout of timestamps on disk.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct Time {
    /// The timestamp using Unix time convention.
    pub timestamp: I64<LE>,

    /// The number of nanoseconds past the beginning of the second denoted in `timestamp`.
    pub nanoseconds: U32<LE>,
}
const_assert_eq!(std::mem::size_of::<Time>(), 12);
