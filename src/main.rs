#[macro_use]
extern crate lazy_static;

use std::env::temp_dir;
use std::time::{SystemTime, UNIX_EPOCH};

use bytes::{Buf, BufMut, BytesMut};
use rocksdb::{DB, Direction, Error, IteratorMode, Options};

lazy_static! {
    static ref PREFIX_META: &'static [u8] = b"m";
    static ref PREFIX_DATA: &'static [u8] = b"d";
}

fn main() {
    let path = temp_dir().as_path().join("test-cedar-rs");
    let path = path.to_str().unwrap();
    Database::destroy(path).unwrap();
    let db = Database::open(path).unwrap();
    println!("open database: {}", db.path);

    // let mut m = Meta::new(1);
    // m.count = 20;
    // let mut buf = m.get_bytes();
    // println!("{:?}", m);
    // println!("{:?}", buf);
    // let mut m2 = Meta::from_bytes(buf.as_mut());
    // println!("{:?}", m2);
    //
    // benchmark_test_case("save meta / 10_0000 times", || {
    //     for i in 0..10_0000 {
    //         let m = Meta::new(i);
    //         db.save_meta(&format!("key_{}", i), &m);
    //     }
    // });
    //
    // // db.for_each_key(|k, v| println!("{:?} = {:?}", k, v));
    // let mut counter = 0;
    // db.for_each_key(|k, v| counter = counter + 1);
    // println!("counter={}", counter);
    //
    // benchmark_test_case("for each key with limit 1 / 10_0000 times", || {
    //     let mut counter = 0;
    //     for i in 0..10_0000 {
    //         db.for_each_key_with_limit(1, |k, v| counter = counter + 1)
    //     }
    //     println!("counter={}", counter);
    // });

    let mut m = Meta::new(123);
    m.count = 456;
    m.extra = Some("hello,world".as_bytes().to_vec());
    db.save_meta("hello", &m).unwrap();
    let mut m2 = db.get_meta("hello").unwrap().unwrap();
    println!("{:?}", m2);
    m2.encode_list_extra(8888, 9999);
    db.save_meta("world", &m2).unwrap();
    let m3 = db.get_meta("world").unwrap().unwrap();
    println!("{:?} {:?}", m3, m3.decode_list_extra());

    db.for_each_key(|k, m| println!("{} = {:?}", k, m))
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

    pub fn destroy(path: &str) -> Result<(), Error> {
        DB::destroy(&Options::default(), path)
    }

    pub fn save_meta(&self, key: &str, meta: &Meta) -> Result<(), Error> {
        self.db.put(encode_meta_key(key), meta.get_bytes())
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<Meta>, Error> {
        self.db.get(encode_meta_key(key))
            .map(|v| v.map(|v| Meta::from_bytes(v.as_slice())))
    }

    pub fn for_each_key<F>(&self, mut f: F) where F: FnMut(&str, &Meta) {
        let iter = self.db.iterator(IteratorMode::From(*PREFIX_META, Direction::Forward));
        for (key, value) in iter {
            f(decode_meta_key(key.as_ref()).as_str(), &Meta::from_bytes(value.as_ref()))
        }
    }

    pub fn for_each_key_with_limit<F>(&self, limit: usize, mut f: F) where F: FnMut(&str, &Meta) {
        let mut counter: usize = 0;
        let iter = self.db.iterator(IteratorMode::From(*PREFIX_META, Direction::Forward));
        for (key, value) in iter {
            counter = counter + 1;
            if counter > limit {
                break;
            }
            f(decode_meta_key(key.as_ref()).as_str(), &Meta::from_bytes(value.as_ref()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Meta {
    pub id: u64,
    pub count: u64,
    pub extra: Option<Vec<u8>>,
}

impl Meta {
    pub fn new(id: u64) -> Meta {
        Meta { id, count: 0, extra: None }
    }

    pub fn from_bytes(input: &[u8]) -> Meta {
        let mut buf = input;
        let id = buf.get_u64();
        let count = buf.get_u64();
        let extra = if buf.remaining() > 0 { Some(buf.bytes().to_vec()) } else { None };
        Meta { id, count, extra }
    }

    pub fn get_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(16);
        buf.put_u64(self.id);
        buf.put_u64(self.count);
        if let Some(b) = &self.extra { buf.put_slice(b) }
        buf
    }

    pub fn decode_list_extra(&self) -> Option<(i64, i64)> {
        if let Some(b) = &self.extra {
            let mut buf = b.as_slice();
            let left = buf.get_i64();
            let right = buf.get_i64();
            Some((left, right))
        } else { None }
    }
    pub fn encode_list_extra(&mut self, left: i64, right: i64) {
        let mut buf = BytesMut::with_capacity(16);
        buf.put_i64(left);
        buf.put_i64(right);
        self.extra = Some(buf.to_vec());
    }
}

pub fn encode_meta_key(key: &str) -> BytesMut {
    let mut buf = BytesMut::with_capacity(9);
    buf.put_slice(*PREFIX_META);
    buf.put_slice(key.as_bytes());
    buf
}

pub fn decode_meta_key(key: &[u8]) -> String {
    String::from_utf8(key[1..].to_vec()).unwrap()
}
