//! NoSQL embedded database on top of RocksDB.
//!
//! ## Usage
//!
//! ```
//! use simpledb::Database;
//!
//! // open a database
//! let db = Database::open("./target/path/to/database").unwrap();
//! // left push a value to a list
//! db.list_left_push("key", "value".as_bytes()).unwrap();
//! ```
//!
//! ## Supported Data Type
//! - **map**: store field/value pairs, includes the following operations with `map_` prefix: `get`, `put`, `delete`, `count`, `for_each`, `items`.
//! - **set**: store unique values, includes the following operations with `set_` prefix: `add`, `is_member`, `delete`, `count`, `for_each`, `items`.
//! - **list**: store ordered values, includes the following operations with `list_` prefix: `left_push`, `left_pop`, `right_push`, `right_pop`, `count`, `for_each`, `items`.
//! - **sorted list**: store sorted score/value pairs, includes the following operations with `sorted_list_` prefix: `add`, `left_pop`, `right_pop`, `count`, `for_each`, `items`.
//! - **sorted set**: store sorted score/value pairs, includes the following operations with `sorted_set_` prefix: `add`, `is_member`, `delete`, `left`, `right`, `for_each`, `items`.
//! - Notes: the difference between `sorted list` and `sorted set` is `list` allow the same members, `set` does not allow the same members.

mod database;

/// Encoding utilities.
pub mod encoding;

pub use database::{Database, Options, Result};
pub use encoding::{BytesComparableScore, KeyMeta, KeyType};

/// Re-exports the rocksdb crate.
pub mod rocksdb {
    pub use rocksdb::*;
}
