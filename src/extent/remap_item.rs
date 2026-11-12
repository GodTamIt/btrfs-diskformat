use static_assertions::const_assert_eq;
use zerocopy::little_endian::U64 as U64LE;
use zerocopy_derive::*;

/// The observed address prefix of an experimental `REMAP` item.
///
/// Any following payload is unmodeled. This prefix does not establish key ranges,
/// mapping direction, or the full item format.
///
/// # Status
///
/// Experimental; the format is not finalized.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/Status.html#experimental-features>
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct RemapItemHeader {
    /// The address recorded in the observed prefix.
    pub address: U64LE,
}
const_assert_eq!(core::mem::size_of::<RemapItemHeader>(), 8);

/// The observed address prefix of an experimental `REMAP_BACKREF` item.
///
/// Any following payload is unmodeled. This prefix does not establish key ranges,
/// mapping direction, or the full item format.
///
/// # Status
///
/// Experimental; the format is not finalized.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/Status.html#experimental-features>
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct RemapBackrefHeader {
    /// The address recorded in the observed prefix.
    pub address: U64LE,
}
const_assert_eq!(core::mem::size_of::<RemapBackrefHeader>(), 8);
