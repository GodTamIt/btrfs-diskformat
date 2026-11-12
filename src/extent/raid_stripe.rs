use static_assertions::const_assert_eq;
use zerocopy::little_endian::U64 as U64LE;
use zerocopy_derive::*;

/// A device and physical address mapping in the experimental RAID-stripe tree.
///
/// This is separate from the chunk mapping's [`Stripe`](crate::Stripe).
///
/// # Status
///
/// Experimental; the format is not finalized.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html#mkfs-feature-raid-stripe-tree>
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct RaidStride {
    /// The device ID.
    pub devid: U64LE,

    /// The physical byte address on the device.
    pub physical: U64LE,
}
const_assert_eq!(core::mem::size_of::<RaidStride>(), 16);

/// Device and physical address mappings for an experimental RAID-stripe tree item.
///
/// The `RAID_STRIPE` payload is an array of strides beginning at byte zero. Its stride
/// count is determined by the item payload length; there is no count header.
/// This is separate from the chunk mapping's [`Stripe`](crate::Stripe).
///
/// # Status
///
/// Experimental; the format is not finalized.
///
/// # Resources
///
/// * <https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html#mkfs-feature-raid-stripe-tree>
#[derive(IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct RaidStripe {
    /// The device and physical address mappings comprising the item payload.
    pub strides: [RaidStride],
}
