use {
    crate::Stripe,
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U16, U32, U64},
};

/// This structure contains the mapping from a virtualized usable byte range within the backing
/// storage to a set of one or more stripes on individual backing devices. In addition to the
/// mapping, hints on optimal I/O parameters for this chunk. It is associated with `CHUNK_ITEM`.
///
/// Although the structure definition only contains one stripe member, `CHUNK_ITEM` contain as
/// many struct [`Stripe`] structures as specified in the [`num_stripes`] and [`sub_stripes`]
/// fields.
///
/// [`Stripe`]: crate::Stripe
/// [`num_stripes`]: Chunk::num_stripes
/// [`sub_stripes`]: Chunk::sub_stripes
#[derive(Copy, Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct Chunk {
    /// The size of this chunk, in bytes.
    pub length: U64<LE>,

    /// The object ID of the root referencing this chunk. This is always the ID of an extent root.
    pub owner: U64<LE>,

    /// The replication stripe length.
    pub stripe_len: U64<LE>,

    /// Flags indicating allocation type and replication policy.
    pub r#type: U64<LE>,

    /// The optimal I/O alignment for this chunk.
    pub io_align: U32<LE>,

    /// The optimal I/O width for this chunk.
    pub io_width: U32<LE>,

    /// The minimal I/O size for this chunk.
    pub sector_size: U32<LE>,

    /// The number of replication stripes.
    pub num_stripes: U16<LE>,

    /// The number of sub-stripes. This is only used for RAID-10.
    pub sub_stripes: U16<LE>,

    /// The first of one or more stripes that map to device extents.
    pub stripe: Stripe,
}
const_assert_eq!(core::mem::size_of::<Chunk>(), 80);
