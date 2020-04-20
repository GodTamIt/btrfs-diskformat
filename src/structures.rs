use {
    crate::constants::*,
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U16, U32, U64},
};

/// An inline array of bytes representing a UUID within the filesystem.
pub type UuidBytes = [u8; 16];

/// The primary superblock is located at `constants::PRIMARY_SUPERBLOCK_ADDR`.
/// There are additional copies of the superblock located at
/// `constants::SUPERBLOCK_ADDRS`, if those addresses are valid.
///
/// The primary superblock must be valid for btrfs to identify a filesystem.
///
/// Resources:
///     * https://btrfs.wiki.kernel.org/index.php/Data_Structures#btrfs_super_block
///     * https://btrfs.wiki.kernel.org/index.php/On-disk_Format#Superblock
#[derive(Clone, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct SuperBlock {
    /// Checksum of everything past this field.
    pub csum: [u8; CSUM_SIZE],

    /// Filesystem UUID.
    pub fsid: [u8; FSID_SIZE],

    /// The physical address of this block.
    pub bytenr: U64<LE>,

    /// Flags
    pub flags: U64<LE>,

    /// The magic must be equal to "_BHRfS_M" in ASCII.
    pub magic: U64<LE>,

    /// The generation of the superblock. In SSD mode, not all superblocks may
    /// be updated, so the latest generation superblock should be used.
    pub generation: U64<LE>,

    /// The logical address of the root tree's root.
    pub root: U64<LE>,

    /// The logical address of the chunk tree's root.
    pub chunk_root: U64<LE>,

    /// The logical address of the log tree's root.
    pub log_root: U64<LE>,

    /// TODO: find out what this is!
    pub log_root_transid: U64<LE>,

    /// TODO: document this!
    pub total_bytes: U64<LE>,

    pub bytes_used: U64<LE>,

    /// The root directory's object ID, which is typically 6.
    pub root_dir_objectid: U64<LE>,

    /// The number of devices the current filesystem spans.
    pub num_devices: U64<LE>,

    /// The size of a sector.
    pub sectorsize: U32<LE>,

    pub nodesize: U32<LE>,

    /// This is currently unused.
    pub leafsize: U32<LE>,

    pub stripesize: U32<LE>,

    /// The size of `sys_chunk_array` found in the superblock.
    pub sys_chunk_array_size: U32<LE>,

    pub chunk_root_generation: U64<LE>,

    pub compat_flags: U64<LE>,

    /// Only implementations that support these flags can write to the filesystem.
    pub compat_ro_flags: U64<LE>,

    /// Only implementations that support these flags can use the filesystem.
    pub incompat_flags: U64<LE>,

    /// The checksum type.
    ///
    /// This should correspond with a value from `constants::ChecksumType`.
    pub csum_type: U16<LE>,

    pub root_level: u8,

    pub chunk_root_level: u8,

    pub log_root_level: u8,

    pub dev_item: DevItem,

    /// The label represented as an ASCII C string. May not contain '/' or '\\'.
    pub label: [u8; LABEL_SIZE],

    pub cache_generation: U64<LE>,

    pub uuid_tree_generation: U64<LE>,

    /// Reserved for extensibility.
    pub _reserved: [U64<LE>; 30],

    pub sys_chunk_array: [u8; MAX_SYSTEM_CHUNK_ARRAY_SIZE],

    pub super_roots: [RootBackup; NUM_BACKUP_ROOTS],

    pub _unused: [u8; 565],
}
const_assert_eq!(std::mem::size_of::<SuperBlock>(), 4096);

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

/// Represents a complete block device.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct DevItem {
    /// The internal btrfs device id.
    ///
    /// This should match the devid found in the filesystem's list of `Device`s.
    pub devid: U64<LE>,

    /// The size of the device.
    pub total_bytes: U64<LE>,

    /// The bytes in use by the filesystem on the device.
    pub bytes_used: U64<LE>,

    /// The optimal I/O alignment for this device.
    pub io_align: U32<LE>,

    /// The optimal I/O width for this device.
    pub io_width: U32<LE>,

    /// The minimum I/O size for this device.
    pub sector_size: U32<LE>,

    /// The type and info for this device.
    pub r#type: U64<LE>,

    /// The expected generation for this device.
    pub generation: U64<LE>,

    /// The starting byte of this partition on the device, to allow for stripe
    /// alignment.
    pub start_offset: U64<LE>,

    /// Grouping information for allocation decisions.
    pub dev_group: U32<LE>,

    /// The seek speed of the device on a scale from 0 to 100, where 100 is the
    /// fastest.
    pub seek_speed: u8,

    /// The bandwidth of the device on a scale from 0 to 100, where 100 is the
    /// fastest.
    pub bandwith: u8,

    /// The generated UUID for this device.
    pub uuid: UuidBytes,

    /// The UUID of the filesystem that owns this device.
    pub fsid: UuidBytes,
}
const_assert_eq!(std::mem::size_of::<DevItem>(), 98);

/// References a subvolume filesystem tree root. This is used for both forward and
/// backward root references.
///
/// The name of the tree is stored after the end of the struct.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct RootRef {
    /// The subtree ID.
    pub dirid: U64<LE>,

    /// The directory sequence number of the subtree entry.
    pub sequence: U64<LE>,

    /// The length of the subtree name, stored after this field.
    pub name_len: U16<LE>,
}
const_assert_eq!(std::mem::size_of::<RootRef>(), 18);

/// Maps physical extents on an individual backing device to a chunk. This extent
/// may be the only one for a particular chunk or one of several.
///
/// It is associated with the `DEV_ITEM` item. This structure is never used
/// outside of this item.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct DevExtent {
    /// The object ID of the chunk tree that owns this extent.
    pub chunk_tree: U64<LE>,

    /// The object ID of the chunk item that references this extent.
    pub chunk_objectid: U64<LE>,

    /// The offset of the chunk item that references this extent.
    pub chunk_offset: U64<LE>,

    /// The length of this extent, in bytes.
    pub length: U64<LE>,

    /// The UUID of the chunk tree that owns this extent.
    pub chunk_tree_uuid: UuidBytes,
}
const_assert_eq!(std::mem::size_of::<DevExtent>(), 48);

// TODO: fix documentation!
/// This structure contains the count for a shared back reference for a file data extent.
///
/// Follows a btrfs_extent_inline_ref of type BTRFS_SHARED_DATA_REF_KEY within an EXTENT_ITEM item. It is never used separately outside of the item body.
/// It immediately follows the btrfs_extent_inline_ref structure that contains the byte offset of the metadata leaf block that contains the EXTENT_DATA item that references this extent.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct SharedDataRef {
    pub count: U32<LE>,
}
const_assert_eq!(std::mem::size_of::<SharedDataRef>(), 4);

/// Defines the location, properties, and usage of a block group.
///
/// It is associated with the `BLOCK_GROUP_ITEM` and is never used outside of this
/// item.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
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

// TODO: fix documentation!
/// This structure contains the mapping from a virtualized usable byte range within the backing storage to a set of one or more stripes on individual backing devices. In addition to the mapping, hints on optimal I/O parameters for this chunk. It is associated with CHUNK_ITEM items.
///
/// Although the structure definition only contains one stripe member, CHUNK_ITEM items contain as many struct btrfs_stripe structures as specified in the num_stripes and sub_stripes fields.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct Chunk {
    /// The size of this chunk, in bytes.
    pub length: U64<LE>,

    /// The object ID of the root referencing this chunk. This is always the ID of
    /// an extent root.
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
const_assert_eq!(std::mem::size_of::<Chunk>(), 80);

/// This structure is used to define the backing device storage that compose a
/// `Chunk`.
///
/// It is associated with the chunk item and is never used outside of this item.
#[derive(Clone, Debug, AsBytes, FromBytes, Unaligned)]
#[repr(C, packed)]
pub struct Stripe {
    /// The ID of the device that contains this stripe.
    pub devid: U64<LE>,

    /// Location of the start of the stripe, in bytes.
    ///
    /// The length is determined by the `stripe_len` field of the associated
    /// `Chunk`.
    pub offset: U64<LE>,

    /// UUID of the device that contains this stripe.
    ///
    /// This can be used to confirm that the correct device has been retrieved.
    pub dev_uuid: UuidBytes,
}
const_assert_eq!(std::mem::size_of::<Stripe>(), 32);
