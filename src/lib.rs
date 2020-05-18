#[macro_use]
extern crate lazy_static;

mod database;
pub mod encoding;

pub use database::{Database, Options};
pub use encoding::{BytesComparableScore, KeyMeta, KeyType};
