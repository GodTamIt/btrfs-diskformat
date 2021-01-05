//! This file contains common type traits and aliases used within structures.

/// An inline array of bytes representing a UUID within the filesystem.
pub type UuidBytes = [u8; 16];

/// Types whose internal integrity are verified by checksum.
pub trait Checksummed {
    /// The value of the checksum field in the structure.
    fn checksum(&self) -> &[u8];

    /// The bytes of the structure used to calculate the checksum.
    fn bytes_to_checksum(&self) -> &[u8];
}