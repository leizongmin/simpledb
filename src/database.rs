use rocksdb::{DB, Direction, Error, IteratorMode, Options};

use crate::encoding::has_prefix;

use super::encoding::{decode_meta_key, encode_meta_key, Meta, PREFIX_META};

pub struct Database {
    pub path: String,
    pub db: DB,
    next_key_id: u64,
}

impl Database {
    pub fn open(path: &str) -> Result<Database, Error> {
        let db = DB::open_default(path)?;
        let mut db = Database {
            path: String::from(path),
            db,
            next_key_id: 1,
        };
        db.after_open();
        Ok(db)
    }

    pub fn destroy(path: &str) -> Result<(), Error> {
        DB::destroy(&Options::default(), path)
    }

    pub fn after_open(&mut self) {
        let mut last_key_id: u64 = 0;
        self.for_each_key(|_, m| {
            last_key_id = m.id;
            true
        });
        self.next_key_id = last_key_id + 1;
    }

    fn prefix_iterator<F>(&self, prefix: &[u8], mut f: F) where F: FnMut(Box<[u8]>, Box<[u8]>) -> bool {
        let iter = self.db.iterator(IteratorMode::From(prefix, Direction::Forward));
        for (k, v) in iter {
            if !has_prefix(prefix, k.as_ref()) {
                break;
            }
            if !f(k, v) {
                break;
            }
        }
    }

    pub fn save_meta(&self, key: &str, meta: &Meta) -> Result<(), Error> {
        self.db.put(encode_meta_key(key), meta.get_bytes())
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<Meta>, Error> {
        self.db.get(encode_meta_key(key))
            .map(|v| v.map(|v| Meta::from_bytes(v.as_slice())))
    }

    pub fn get_or_create_meta(&mut self, key: &str) -> Result<Option<Meta>, Error> {
        let m = self.get_meta(key)?;
        if let None = m {
            let m = Meta::new(self.next_key_id);
            self.next_key_id += 1;
            self.save_meta(key, &m)?;
            Ok(Some(m))
        } else {
            Ok(m)
        }
    }

    pub fn for_each_key<F>(&self, mut f: F) -> usize
        where F: FnMut(&str, &Meta) -> bool {
        let mut counter: usize = 0;
        self.prefix_iterator(*PREFIX_META, |k, v| {
            counter = counter + 1;
            f(decode_meta_key(k.as_ref()).as_str(), &Meta::from_bytes(v.as_ref()))
        });
        counter
    }

    pub fn for_each_key_with_limit<F>(&self, limit: usize, mut f: F) -> usize
        where F: FnMut(&str, &Meta) -> bool {
        let mut counter: usize = 0;
        self.prefix_iterator(*PREFIX_META, |k, v| {
            counter = counter + 1;
            if counter > limit {
                false
            } else {
                f(decode_meta_key(k.as_ref()).as_str(), &Meta::from_bytes(v.as_ref()))
            }
        });
        counter
    }
}
