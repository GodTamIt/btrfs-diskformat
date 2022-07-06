use {
    crate::{InodeItem, Key, Time, UuidBytes},
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U32, U64},
};

/// Defines the location and parameters of the root of a b-tree.
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct RootItem {
    pub inode: InodeItem,

    pub generation: U64<LE>,

    pub root_dirid: U64<LE>,

    pub bytenr: U64<LE>,

    /// Currently unused. Always 0.
    pub byte_limit: U64<LE>,

    /// Currently unused.
    pub bytes_used: U64<LE>,

    /// The transaction ID of the last transaction that created a snapshot of this root.
    pub last_snapshot: U64<LE>,

    pub flags: U64<LE>,

    /// Only 0 or 1. Historically contained a reference count.
    pub refs: U32<LE>,

    /// Contains the key of the last dropped item during subvolume removal or relocation.
    ///
    /// Value will be zeroed out otherwise.
    pub drop_progress: Key,

    /// The tree level of the node referenced in [drop_progress](RootItem::drop_progress).
    pub drop_level: u8,

    /// The height of this root's tree.
    pub level: u8,

    /// Value to help determine whether this root has been modified by an older btrfs
    /// implementation.
    ///
    /// If the value is equal to [generation], the fields below are valid. Otherwise, this indicates
    /// the fields are invalid but recoverable.
    ///
    /// [generation]: RootItem::generation
    pub generation_v2: U64<LE>,

    /// The subvolume's UUID.
    pub uuid: UuidBytes,

    /// The parent's subvolume UUID.
    ///
    /// This is used during send/receive.
    pub parent_uuid: UuidBytes,

    /// The received UUID.
    ///
    /// This is used during send/receive.
    pub received_uuid: UuidBytes,

    /// The transaction ID of the last transaction that modified the tree.
    ///
    /// Note: some operations like internal caches or relocation will not update this value.
    pub ctransid: U64<LE>,

    /// The transaction ID of the transaction that created the tree.
    pub otransid: U64<LE>,

    /// The transaction ID for the transaction that sent this subvolume.
    ///
    /// This value is non-zero for a received subvolume.
    pub stransid: U64<LE>,

    /// The transaction ID for the transaction that received this subvolume.
    ///
    /// This value is non-zero for a received subvolume.
    pub rtransid: U64<LE>,

    /// The timestamp of the [`ctransid`](RootItem::ctransid).
    pub ctime: Time,

    /// The timestamp of the [`otransid`](RootItem::otransid).
    pub otime: Time,

    /// The timestamp of the [`stransid`](RootItem::stransid).
    pub stime: Time,

    /// The timestamp of the [`rtransid`](RootItem::rtransid).
    pub rtime: Time,

    /// Currently unused. Reserved for future use.
    pub _unused: [u64; 8],
}
const_assert_eq!(core::mem::size_of::<RootItem>(), 439);
