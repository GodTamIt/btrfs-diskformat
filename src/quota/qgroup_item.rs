use bitflags::bitflags;
use static_assertions::const_assert_eq;
use zerocopy::little_endian::U64 as U64LE;
use zerocopy_derive::*;

/// The common prefix of quota status items, including shorter legacy items.
///
/// Legacy 32-byte items must use this type rather than [`QgroupStatusItem`].
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct QgroupStatusItemHeader {
    /// The quota status format version.
    pub version: U64LE,

    /// The quota status generation.
    pub generation: U64LE,

    /// Raw flags described by [`QgroupStatusFlags`].
    pub flags: U64LE,

    /// The quota rescan progress.
    pub rescan: U64LE,
}
const_assert_eq!(core::mem::size_of::<QgroupStatusItemHeader>(), 32);

/// A 40-byte quota status item, including the simple quota enable generation.
///
/// Use [`QgroupStatusItemHeader`] for shorter legacy items.
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct QgroupStatusItem {
    /// The quota status format version.
    pub version: U64LE,

    /// The quota status generation.
    pub generation: U64LE,

    /// Raw flags described by [`QgroupStatusFlags`].
    pub flags: U64LE,

    /// The quota rescan progress.
    pub rescan: U64LE,

    /// The generation when simple quotas were enabled.
    pub enable_gen: U64LE,
}
const_assert_eq!(core::mem::size_of::<QgroupStatusItem>(), 40);

/// Space accounting for a quota group.
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct QgroupInfoItem {
    /// The accounting generation.
    pub generation: U64LE,

    /// The referenced space, in bytes.
    pub referenced: U64LE,

    /// The compressed referenced space, in bytes.
    pub referenced_compressed: U64LE,

    /// The exclusively owned space, in bytes.
    pub exclusive: U64LE,

    /// The compressed exclusively owned space, in bytes.
    pub exclusive_compressed: U64LE,
}
const_assert_eq!(core::mem::size_of::<QgroupInfoItem>(), 40);

/// Space limits and reservations for a quota group.
#[derive(Copy, Clone, Debug, Hash, IntoBytes, FromBytes, Unaligned, KnownLayout, Immutable)]
#[repr(C, packed)]
pub struct QgroupLimitItem {
    /// Raw flags described by [`QgroupLimitFlags`].
    pub flags: U64LE,

    /// The maximum referenced space, in bytes.
    pub max_rfer: U64LE,

    /// The maximum exclusively owned space, in bytes.
    pub max_excl: U64LE,

    /// Reserved referenced space, in bytes. Documented as unused.
    pub rsv_rfer: U64LE,

    /// Reserved exclusively owned space, in bytes. Documented as unused.
    pub rsv_excl: U64LE,
}
const_assert_eq!(core::mem::size_of::<QgroupLimitItem>(), 40);

bitflags! {
    /// Quota limit flags documented in the legacy item layout.
    pub struct QgroupLimitFlags: u64 {
        /// The maximum referenced space limit is enabled.
        const MAX_RFER = 0x1;
        /// The maximum exclusive space limit is enabled.
        const MAX_EXCL = 0x2;
        /// Reserved referenced space flag. Documented as unused.
        const RSV_RFER = 0x4;
        /// Reserved exclusive space flag. Documented as unused.
        const RSV_EXCL = 0x8;
        /// Compressed referenced space limit flag. Documented as unused.
        const RFER_CMPR = 0x10;
        /// Compressed exclusive space limit flag. Documented as unused.
        const EXCL_CMPR = 0x20;
    }
}

bitflags! {
    /// Quota accounting status flags.
    pub struct QgroupStatusFlags: u64 {
        /// Quota accounting is enabled.
        const ON = 0x1;
        /// A quota rescan is in progress (displayed as SCANNING by dump-tree).
        const RESCAN = 0x2;
        /// Quota accounting is inconsistent.
        const INCONSISTENT = 0x4;
        /// Simple quota accounting is enabled.
        const SIMPLE_MODE = 0x8;
    }
}
