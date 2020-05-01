#[macro_use]
extern crate lazy_static;

use std::borrow::Borrow;
use std::env::temp_dir;
use std::ops::Sub;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use bytes::{Buf, BufMut, BytesMut};
use rocksdb::{DB, Direction, Error, IteratorMode, Options};

lazy_static! {
    static ref PREFIX_META: &'static [u8] = "m".as_bytes();
    static ref PREFIX_DATA: &'static [u8] = "d".as_bytes();
}

fn main() {
    let path = temp_dir().as_path().join("test-cedar-rs");
    let db = Database::open(path.to_str().unwrap()).unwrap();
    println!("open database: {}", db.path);

    let mut m = Meta::new(1);
    m.count = 20;
    let mut buf = m.get_bytes();
    println!("{:?}", m);
    println!("{:?}", buf);
    let mut m2 = Meta::from_bytes(buf.as_mut());
    println!("{:?}", m2);

    benchmark_test_case("save meta / 10_0000 times", || {
        for i in 0..10_0000 {
            let m = Meta::new(i);
            db.save_meta(&m);
        }
    });

    // db.for_each_key(|k, v| println!("{:?} = {:?}", k, v));
    let mut counter = 0;
    db.for_each_key(|k, v| counter = counter + 1);
    println!("counter={}", counter);

    benchmark_test_case("for each key with limit 1 / 10_0000 times", || {
        let mut counter = 0;
        for i in 0..10_0000 {
            db.for_each_key_with_limit(1, |k, v| counter = counter + 1)
        }
        println!("counter={}", counter);
    });
}

pub fn benchmark_test_case<F>(title: &str, mut f: F) where F: FnMut() {
    println!("start {}...", title);
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    f();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    println!("{}, spent={}ms", title, end - start);
}

pub struct Database {
    pub path: String,
    pub db: DB,
}

impl Database {
    pub fn open(path: &str) -> Result<Database, Error> {
        let db = DB::open_default(path)?;
        Ok(Database {
            path: String::from(path),
            db,
        })
    }

    pub fn save_meta(&self, meta: &Meta) -> Result<(), Error> {
        self.db.put(encode_meta_key(meta.id), meta.get_bytes())
    }

    pub fn for_each_key<F>(&self, mut f: F) where F: FnMut(Box<[u8]>, Box<[u8]>) {
        let iter = self.db.iterator(IteratorMode::From(*PREFIX_META, Direction::Forward));
        for (key, value) in iter {
            f(key, value);
        }
    }

    pub fn for_each_key_with_limit<F>(&self, limit: usize, mut f: F) where F: FnMut(Box<[u8]>, Box<[u8]>) {
        let mut counter: usize = 0;
        let iter = self.db.iterator(IteratorMode::From(*PREFIX_META, Direction::Forward));
        for (key, value) in iter {
            counter = counter + 1;
            if counter > limit {
                break;
            }
            f(key, value);
        }
    }
}

#[derive(Debug)]
pub struct Meta {
    pub id: u64,
    pub count: u64,
}

impl Meta {
    pub fn new(id: u64) -> Meta {
        Meta { id, count: 0 }
    }

    pub fn from_bytes(input: &[u8]) -> Meta {
        let mut buf = input;
        let id = buf.get_u64();
        let count = buf.get_u64();
        Meta { id, count }
    }

    pub fn get_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(16);
        buf.put_u64(self.id);
        buf.put_u64(self.count);
        buf
    }
}

pub fn encode_meta_key(id: u64) -> BytesMut {
    let mut buf = BytesMut::with_capacity(9);
    buf.put_slice(*PREFIX_META);
    buf.put_u64(id);
    buf
}
