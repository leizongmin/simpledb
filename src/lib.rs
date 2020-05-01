#[macro_use]
extern crate lazy_static;

pub mod database;
pub mod encoding;

#[cfg(test)]
mod tests {
    use std::env::temp_dir;
    use std::time::{SystemTime, UNIX_EPOCH};

    use rand::prelude::ThreadRng;
    use rand::Rng;

    use crate::encoding::KeyType;

    use super::database::Database;

    fn benchmark_test_case<F>(title: &str, mut f: F)
        where
            F: FnMut(),
    {
        println!("start {}...", title);
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        f();
        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("{}, spent={}ms", title, end - start);
    }

    fn get_random_database_path() -> String {
        let r: i32 = rand::thread_rng().gen();
        let path = temp_dir().as_path().join(format!(
            "test-cedar-rs-{}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            r
        ));
        String::from(path.to_str().unwrap())
    }

    fn open_database() -> Database {
        let path = get_random_database_path();
        Database::destroy(&path).unwrap();
        open_database_with_path(&path)
    }

    fn open_database_with_path(path: &str) -> Database {
        let db = Database::open(path).unwrap();
        println!("open database: {}", db.path);
        db
    }

    fn dump_database_meta(db: &Database) {
        println!("dump_database_meta:");
        db.for_each_key(|k, m| {
            println!("key: {:?}\t value: {:?}", k, m);
            true
        });
    }

    fn dump_database_data(db: &Database, key: &str) {
        println!("dump_database_data:");
        db.for_each_data(key, |k, m| {
            println!("key: {:?}\t value: {:?}", k, m);
            true
        });
    }

    fn vec_to_str(vec: Vec<u8>) -> String {
        String::from_utf8(vec).unwrap()
    }

    #[test]
    fn test_get_or_save_meta() {
        let path = get_random_database_path();
        {
            let mut db = open_database_with_path(&path);
            let m1 = db.get_or_create_meta("aaa", KeyType::Map).unwrap().unwrap();
            assert_eq!(1, m1.id);
            let m2 = db
                .get_or_create_meta("bbb", KeyType::List)
                .unwrap()
                .unwrap();
            assert_eq!(2, m2.id);

            let m3 = db.get_meta("ccc").unwrap();
            assert!(m3.is_none());
            let m4 = db.get_meta("aaa").unwrap();
            assert!(m4.is_some());

            dump_database_meta(&db);
        }
        {
            let mut db = open_database_with_path(&path);
            dump_database_meta(&db);

            let m1 = db.get_or_create_meta("aaa", KeyType::Map).unwrap().unwrap();
            assert_eq!(1, m1.id);
            assert_eq!(KeyType::Map, m1.key_type);
            let m2 = db
                .get_or_create_meta("bbb", KeyType::List)
                .unwrap()
                .unwrap();
            assert_eq!(2, m2.id);
            assert_eq!(KeyType::List, m2.key_type);
            let m3 = db.get_meta("ccc").unwrap();
            assert!(m3.is_none());

            let m4 = db.get_or_create_meta("ccc", KeyType::Set).unwrap().unwrap();
            assert_eq!(3, m4.id);
            assert_eq!(KeyType::Set, m4.key_type);

            dump_database_meta(&db);
        }
    }

    #[test]
    fn test_map() {
        let path = get_random_database_path();
        {
            let mut db = open_database_with_path(&path);
            let key = "hello";

            assert_eq!(0, db.map_count(key).unwrap());
            assert!(db.map_get(key, "aaa").unwrap().is_none());
            assert!(db.map_get(key, "bbb").unwrap().is_none());
            assert!(db.map_get(key, "ccc").unwrap().is_none());

            db.map_put(key, "aaa", "123".as_bytes()).unwrap();
            db.map_put(key, "bbb", "456".as_bytes()).unwrap();
            db.map_put(key, "ccc", "789".as_bytes()).unwrap();
            assert_eq!(3, db.map_count(key).unwrap());

            dump_database_meta(&db);
            dump_database_data(&db, key);

            assert_eq!("123", vec_to_str(db.map_get(key, "aaa").unwrap().unwrap()));
            assert_eq!("456", vec_to_str(db.map_get(key, "bbb").unwrap().unwrap()));
            assert_eq!("789", vec_to_str(db.map_get(key, "ccc").unwrap().unwrap()));

            let vec = db.map_items(key).unwrap();
            assert_eq!(3, vec.len());
            let (f, v) = vec.get(0).unwrap();
            assert_eq!("aaa", f);
            assert_eq!("123", vec_to_str(v.to_vec()));

            assert_eq!(true, db.map_delete(key, "aaa").unwrap());
            assert_eq!(true, db.map_delete(key, "bbb").unwrap());
            assert_eq!(false, db.map_delete(key, "ddd").unwrap());
            assert_eq!(1, db.map_count(key).unwrap());

            dump_database_meta(&db);
            dump_database_data(&db, key);

            assert_eq!(true, db.map_delete(key, "ccc").unwrap());

            let mut counter = 0;
            db.for_each_key(|k, m| {
                counter += 1;
                true
            });
            assert_eq!(0, counter);
        }
    }
}
