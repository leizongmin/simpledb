use rocksdb::{DB, Direction, Error, IteratorMode, Options, ReadOptions};

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

    fn after_open(&mut self) {
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

    pub fn get_or_create_meta(&mut self, key: &str, key_type: KeyType) -> Result<KeyMeta, Error> {
        let m = self.get_meta(key)?;
        return match m {
            Some(m) => Ok(m),
            None => {
                let m = KeyMeta::new(self.next_key_id, key_type);
                self.next_key_id += 1;
                self.save_meta(key, &m, false)?;
                Ok(m)
            }
        };
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
        let meta = self.get_or_create_meta(key, KeyType::Map)?;
        let full_key = encode_data_key_map_item(meta.id, field);
        self.db.get(full_key)
    }

    pub fn map_put(&mut self, key: &str, field: &str, value: &[u8]) -> Result<(), Error> {
        let mut meta = self.get_or_create_meta(key, KeyType::Map)?;
        let full_key = encode_data_key_map_item(meta.id, field);
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
            let full_key = encode_data_key_map_item(meta.id, field);
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
            F: FnMut(&str, Box<[u8]>) -> bool {
        self.for_each_data(key, |k, v| {
            let k = decode_data_key_map_item(k.as_ref());
            f(&k, v)
        })
    }

    pub fn map_items(&mut self, key: &str) -> Result<Vec<(String, Box<[u8]>)>, Error> {
        let count = self.get_count(key)?;
        let mut vec = Vec::with_capacity(count as u64 as usize);
        self.map_for_each(key, |f, v| {
            vec.push((String::from(f), v));
            true
        })?;
        Ok(vec)
    }

    pub fn set_count(&self, key: &str) -> Result<u64, Error> {
        self.get_count(key)
    }

    pub fn set_add(&mut self, key: &str, value: &[u8]) -> Result<bool, Error> {
        let mut meta = self.get_or_create_meta(key, KeyType::Set)?;
        let full_key = encode_data_key_set_item(meta.id, value);
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
            let full_key = encode_data_key_set_item(meta.id, value);
            Ok(self.db.get(&full_key)?.is_some())
        }
    }

    pub fn set_delete(&mut self, key: &str, value: &[u8]) -> Result<bool, Error> {
        let meta = self.get_meta(key)?;
        if meta.is_none() {
            Ok(false)
        } else {
            let mut meta = meta.unwrap();
            let full_key = encode_data_key_set_item(meta.id, value);
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
            let value = decode_data_key_set_item(k.as_ref());
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

    pub fn list_count(&mut self, key: &str) -> Result<u64, Error> {
        self.get_count(key)
    }

    pub fn list_left_push(&mut self, key: &str, value: &[u8]) -> Result<u64, Error> {
        let mut meta = self.get_or_create_meta(key, KeyType::List)?;
        let (left, right) = meta.decode_list_extra();
        let full_key = encode_data_key_list_item(meta.id, left);
        self.db.put(full_key, value)?;
        meta.encode_list_extra(left - 1, right);
        meta.count += 1;
        self.save_meta(key, &meta, false)?;
        Ok(meta.count)
    }

    pub fn list_right_push(&mut self, key: &str, value: &[u8]) -> Result<u64, Error> {
        let mut meta = self.get_or_create_meta(key, KeyType::List)?;
        let (left, right) = meta.decode_list_extra();
        let full_key = encode_data_key_list_item(meta.id, right);
        self.db.put(full_key, value)?;
        meta.encode_list_extra(left, right + 1);
        meta.count += 1;
        self.save_meta(key, &meta, false)?;
        Ok(meta.count)
    }

    pub fn list_left_pop(&mut self, key: &str) -> Result<Option<Box<[u8]>>, Error> {
        let meta = self.get_meta(key)?;
        if meta.is_some() {
            let mut meta = meta.unwrap();
            let (left, right) = meta.decode_list_extra();
            let full_key = encode_data_key_list_item(meta.id, left + 1);
            match self.db.get(full_key.as_ref())? {
                Some(value) => {
                    meta.encode_list_extra(left + 1, right);
                    meta.count -= 1;
                    self.save_meta(key, &meta, true)?;
                    self.db.delete(full_key.as_ref())?;
                    Ok(Some(Box::from(value)))
                }
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn list_right_pop(&mut self, key: &str) -> Result<Option<Box<[u8]>>, Error> {
        let meta = self.get_meta(key)?;
        if meta.is_some() {
            let mut meta = meta.unwrap();
            let (left, right) = meta.decode_list_extra();
            let full_key = encode_data_key_list_item(meta.id, right - 1);
            match self.db.get(full_key.as_ref())? {
                Some(value) => {
                    meta.encode_list_extra(left, right - 1);
                    meta.count -= 1;
                    self.save_meta(key, &meta, true)?;
                    self.db.delete(full_key.as_ref())?;
                    Ok(Some(Box::from(value)))
                }
                None => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn list_for_each<F>(&mut self, key: &str, mut f: F) -> Result<u64, Error>
        where
            F: FnMut(Box<[u8]>) -> bool {
        self.for_each_data(key, |_, v| {
            f(Box::from(v))
        })
    }

    pub fn list_items(&mut self, key: &str) -> Result<Vec<Box<[u8]>>, Error> {
        let count = self.get_count(key)?;
        let mut vec = Vec::with_capacity(count as u64 as usize);
        self.list_for_each(key, |v| {
            vec.push(v);
            true
        })?;
        Ok(vec)
    }

    pub fn sorted_list_count(&mut self, key: &str) -> Result<u64, Error> {
        self.get_count(key)
    }

    pub fn sorted_list_add(&mut self, key: &str, score: &[u8], value: &[u8]) -> Result<u64, Error> {
        let mut meta = self.get_or_create_meta(key, KeyType::SortedList)?;
        let sequence = meta.decode_sorted_list_extra();
        let full_key = encode_data_key_sorted_list_item(meta.id, score, sequence);
        meta.encode_sorted_list_extra(sequence + 1);
        meta.count += 1;
        self.db.put(full_key, value)?;
        self.save_meta(key, &meta, false)?;
        Ok(meta.count)
    }

    pub fn sorted_list_left_pop(&mut self, key: &str, max_score: Option<&[u8]>) -> Result<Option<(Box<[u8]>, Box<[u8]>)>, Error> {
        let meta = self.get_meta(key)?;
        let mut ret: Option<(Box<[u8]>, Box<[u8]>)> = None;
        if let Some(mut meta) = meta {
            let prefix = encode_data_key(meta.id);
            let mut opts = ReadOptions::default();
            opts.set_prefix_same_as_start(true);
            opts.set_total_order_seek(true);
            let iter = self.db.iterator_opt(IteratorMode::From(&prefix, Direction::Forward), opts);
            for (k, v) in iter {
                if !has_prefix(&prefix, k.as_ref()) {
                    break;
                }
                let score = decode_data_key_sorted_list_item(k.as_ref());
                if let Some(max_score) = max_score {
                    if compare_score_bytes(score, max_score) > 0 {
                        break;
                    }
                }
                meta.count -= 1;
                self.db.delete(k.as_ref())?;
                self.save_meta(key, &meta, true)?;
                ret = Some((Box::from(score), v));
                break;
            }
        }
        Ok(ret)
    }

    pub fn sorted_list_right_pop(&mut self, key: &str, min_score: Option<&[u8]>) -> Result<Option<(Box<[u8]>, Box<[u8]>)>, Error> {
        let meta = self.get_meta(key)?;
        if let Some(mut meta) = meta {
            let prefix = encode_data_key(meta.id);
            let next_prefix = encode_data_key(meta.id + 1);
            let iter = self.db.iterator(IteratorMode::From(&next_prefix, Direction::Reverse));
            for (k, v) in iter {
                if !has_prefix(&prefix, k.as_ref()) {
                    return Ok(None);
                }
                let score = decode_data_key_sorted_list_item(k.as_ref());
                if let Some(min_score) = min_score {
                    if compare_score_bytes(score, min_score) < 0 {
                        return Ok(None);
                    }
                }
                meta.count -= 1;
                self.db.delete(k.as_ref())?;
                self.save_meta(key, &meta, true)?;
                return Ok(Some((Box::from(score), v)));
            }
        }
        Ok(None)
    }

    pub fn sorted_list_for_each<F>(&mut self, key: &str, mut f: F) -> Result<u64, Error>
        where
            F: FnMut((Box<[u8]>, Box<[u8]>)) -> bool {
        self.for_each_data(key, |k, v| {
            let score = decode_data_key_sorted_list_item(k.as_ref());
            f((Box::from(score), Box::from(v)))
        })
    }

    pub fn sorted_list_items(&mut self, key: &str) -> Result<Vec<(Box<[u8]>, Box<[u8]>)>, Error> {
        let count = self.get_count(key)?;
        let mut vec = Vec::with_capacity(count as u64 as usize);
        self.sorted_list_for_each(key, |item| {
            vec.push(item);
            true
        })?;
        Ok(vec)
    }
}
