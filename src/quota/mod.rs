//! Quota accounting items.
//!
//! Status, information, and limit items use key types 240, 242, and 244.
//! Qgroup relations (key type 246) have empty payloads and are encoded in keys.
//!
//! Legacy item layouts and limit flags are documented in
//! <https://raw.githubusercontent.com/btrfs/btrfs-dev-docs/master/tree-items.txt>.
//!
//! For quota semantics, rather than item layouts, see
//! <https://btrfs.readthedocs.io/en/latest/Qgroups.html>.

mod qgroup_item;

pub use qgroup_item::*;
