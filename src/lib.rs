#[macro_use]
extern crate lazy_static;

pub mod database;
pub mod encoding;

#[cfg(test)]
mod tests {
    use std::env::temp_dir;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::encoding::KeyType;

    use super::database::Database;

    static mut CREATED_DB_COUNT: usize = 0;

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
        unsafe { CREATED_DB_COUNT = CREATED_DB_COUNT + 1 }
        let path = temp_dir().as_path().join(format!(
            "test-cedar-rs-{}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            unsafe { CREATED_DB_COUNT }
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
        db.for_each_key(|k, m| {
            println!("key: {:?}\t value: {:?}", k, m);
            true
        });
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

            db.map_put("hello", "aaa", "123".as_bytes()).unwrap();
            db.map_put("hello", "bbb", "456".as_bytes()).unwrap();
            db.map_put("hello", "ccc", "789".as_bytes()).unwrap();

            dump_database_meta(&db);
        }
    }
}
