#[macro_use]
extern crate lazy_static;

mod database;
pub mod encoding;

pub use database::Database;
pub use encoding::{BytesComparableScore, KeyMeta, KeyType};
