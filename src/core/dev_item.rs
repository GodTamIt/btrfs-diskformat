use {
    crate::UuidBytes,
    byteorder::LE,
    static_assertions::const_assert_eq,
    zerocopy::{AsBytes, FromBytes, Unaligned, U32, U64},
};

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
