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
//! - **map**: Store field/value pairs, includes the following operations with `map_` prefix: `get`, `put`, `delete`, `count`, `for_each`, `items`.
//! - **set**: Store unique values, includes the following operations with `set_` prefix: `add`, `is_member`, `delete`, `count`, `for_each`, `items`.
//! - **list**: Store ordered values, includes the following operations with `list_` prefix: `left_push`, `left_pop`, `right_push`, `right_pop`, `count`, `for_each`, `items`.
//! - **sorted list**: Store sorted score/value pairs, includes the following operations with `sorted_list_` prefix: `add`, `left_pop`, `right_pop`, `count`, `for_each`, `items`.
//!

mod database;

/// Encoding utilities.
pub mod encoding;

pub use database::{Database, Options, Result};
pub use encoding::{BytesComparableScore, KeyMeta, KeyType};

/// Re-exports the rocksdb crate.
pub mod rocksdb {
    pub use rocksdb::*;
}
