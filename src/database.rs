use rocksdb::{DB, Direction, Error, IteratorMode, Options};

use crate::encoding::{encode_data_key, has_prefix, KeyType};

use super::encoding::*;

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

    fn prefix_iterator<F>(&self, prefix: &[u8], mut f: F)
        where
            F: FnMut(Box<[u8]>, Box<[u8]>) -> bool,
    {
        let iter = self
            .db
            .iterator(IteratorMode::From(prefix, Direction::Forward));
        for (k, v) in iter {
            if !has_prefix(prefix, k.as_ref()) {
                break;
            }
            if !f(k, v) {
                break;
            }
        }
    }

    pub fn save_meta(&self, key: &str, meta: &KeyMeta, delete_if_empty: bool) -> Result<(), Error> {
        if delete_if_empty && meta.count < 1 {
            self.db.delete(encode_meta_key(key))
        } else {
            self.db.put(encode_meta_key(key), meta.get_bytes())
        }
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<KeyMeta>, Error> {
        self.db
            .get(encode_meta_key(key))
            .map(|v| v.map(|v| KeyMeta::from_bytes(v.as_slice())))
    }

    pub fn get_or_create_meta(
        &mut self,
        key: &str,
        key_type: KeyType,
    ) -> Result<Option<KeyMeta>, Error> {
        let m = self.get_meta(key)?;
        if let None = m {
            let m = KeyMeta::new(self.next_key_id, key_type);
            self.next_key_id += 1;
            self.save_meta(key, &m, false)?;
            Ok(Some(m))
        } else {
            Ok(m)
        }
    }

    pub fn for_each_key<F>(&self, mut f: F) -> usize
        where
            F: FnMut(&str, &KeyMeta) -> bool,
    {
        let mut counter: usize = 0;
        self.prefix_iterator(*PREFIX_META, |k, v| {
            counter = counter + 1;
            f(
                decode_meta_key(k.as_ref()).as_str(),
                &KeyMeta::from_bytes(v.as_ref()),
            )
        });
        counter
    }

    pub fn for_each_key_with_limit<F>(&self, limit: usize, mut f: F) -> usize
        where
            F: FnMut(&str, &KeyMeta) -> bool,
    {
        let mut counter: usize = 0;
        self.prefix_iterator(*PREFIX_META, |k, v| {
            counter = counter + 1;
            if counter > limit {
                false
            } else {
                f(
                    decode_meta_key(k.as_ref()).as_str(),
                    &KeyMeta::from_bytes(v.as_ref()),
                )
            }
        });
        counter
    }

    pub fn for_each_data<F>(&self, key: &str, mut f: F) -> Result<u64, Error>
        where
            F: FnMut(Box<[u8]>, Box<[u8]>) -> bool {
        let meta = self.get_meta(key)?;
        match meta {
            Some(meta) => {
                if meta.count > 0 {
                    let mut counter = 0;
                    self.prefix_iterator(encode_data_key(meta.id).as_ref(), |k, v| {
                        counter += 1;
                        f(k, v)
                    });
                    Ok(counter)
                } else {
                    Ok(0)
                }
            }
            None => Ok(0)
        }
    }

    pub fn get_count(&self, key: &str) -> Result<u64, Error> {
        let meta = self.get_meta(key)?;
        Ok(match meta {
            Some(m) => m.count,
            _ => 0,
        })
    }

    pub fn map_count(&self, key: &str) -> Result<u64, Error> {
        self.get_count(key)
    }

    pub fn map_get(&mut self, key: &str, field: &str) -> Result<Option<Vec<u8>>, Error> {
        let meta = self.get_or_create_meta(key, KeyType::Map)?.unwrap();
        let full_key = encode_data_key_map_field(meta.id, field);
        self.db.get(full_key)
    }

    pub fn map_put(&mut self, key: &str, field: &str, value: &[u8]) -> Result<(), Error> {
        let mut meta = self.get_or_create_meta(key, KeyType::Map)?.unwrap();
        let full_key = encode_data_key_map_field(meta.id, field);
        if self.db.get(&full_key)?.is_none() {
            meta.count += 1;
        }
        self.db.put(&full_key, value)?;
        self.save_meta(key, &meta, false)
    }

    pub fn map_delete(&mut self, key: &str, field: &str) -> Result<bool, Error> {
        let meta = self.get_meta(key)?;
        if meta.is_none() {
            Ok(false)
        } else {
            let mut meta = meta.unwrap();
            let full_key = encode_data_key_map_field(meta.id, field);
            if self.db.get(&full_key)?.is_some() {
                meta.count -= 1;
                self.db.delete(&full_key)?;
                self.save_meta(key, &meta, true)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn map_for_each<F>(&mut self, key: &str, mut f: F) -> Result<u64, Error>
        where
            F: FnMut(&str, &[u8]) -> bool {
        self.for_each_data(key, |k, v| {
            let k = decode_data_key_map_field(k.as_ref());
            f(&k, v.as_ref())
        })
    }

    pub fn map_items(&mut self, key: &str) -> Result<Vec<(String, Vec<u8>)>, Error> {
        let count = self.get_count(key)?;
        let mut vec = Vec::with_capacity(count as u64 as usize);
        self.map_for_each(key, |f, v| {
            vec.push((String::from(f), v.to_vec()));
            true
        })?;
        Ok(vec)
    }

    pub fn set_count(&self, key: &str) -> Result<u64, Error> {
        self.get_count(key)
    }

    pub fn set_add(&mut self, key: &str, value: &[u8]) -> Result<bool, Error> {
        let mut meta = self.get_or_create_meta(key, KeyType::Set)?.unwrap();
        let full_key = encode_data_key_set_value(meta.id, value);
        let mut is_new_item = false;
        if self.db.get(&full_key)?.is_none() {
            meta.count += 1;
            is_new_item = true;
        }
        self.db.put(&full_key, *FILL_EMPTY_DATA)?;
        if is_new_item {
            self.save_meta(key, &meta, false)?;
        }
        Ok(is_new_item)
    }

    pub fn set_is_member(&mut self, key: &str, value: &[u8]) -> Result<bool, Error> {
        let meta = self.get_meta(key)?;
        if meta.is_none() {
            Ok(false)
        } else {
            let meta = meta.unwrap();
            let full_key = encode_data_key_set_value(meta.id, value);
            Ok(self.db.get(&full_key)?.is_some())
        }
    }

    pub fn set_delete(&mut self, key: &str, value: &[u8]) -> Result<bool, Error> {
        let meta = self.get_meta(key)?;
        if meta.is_none() {
            Ok(false)
        } else {
            let mut meta = meta.unwrap();
            let full_key = encode_data_key_set_value(meta.id, value);
            if self.db.get(&full_key)?.is_some() {
                meta.count -= 1;
                self.db.delete(full_key)?;
                self.save_meta(key, &meta, true)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    pub fn set_for_each<F>(&mut self, key: &str, mut f: F) -> Result<u64, Error>
        where
            F: FnMut(Box<[u8]>) -> bool {
        self.for_each_data(key, |k, _| {
            let value = decode_data_key_set_value(k.as_ref());
            f(Box::from(value))
        })
    }

    pub fn set_items(&mut self, key: &str) -> Result<Vec<Box<[u8]>>, Error> {
        let count = self.get_count(key)?;
        let mut vec = Vec::with_capacity(count as u64 as usize);
        self.set_for_each(key, |v| {
            vec.push(v);
            true
        })?;
        Ok(vec)
    }
}
