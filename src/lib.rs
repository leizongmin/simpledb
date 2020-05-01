#[macro_use]
extern crate lazy_static;

pub mod encoding;
pub mod database;

#[cfg(test)]
mod tests {
    use std::env::temp_dir;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::database::Database;

    static mut CREATED_DB_COUNT: usize = 0;

    fn benchmark_test_case<F>(title: &str, mut f: F) where F: FnMut() {
        println!("start {}...", title);
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        f();
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        println!("{}, spent={}ms", title, end - start);
    }

    fn get_random_database_path() -> String {
        unsafe { CREATED_DB_COUNT = CREATED_DB_COUNT + 1 }
        let path = temp_dir().as_path()
            .join(format!("test-cedar-rs-{}", unsafe { CREATED_DB_COUNT }));
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
            let m1 = db.get_or_create_meta("aaa").unwrap().unwrap();
            assert_eq!(1, m1.id);
            let m2 = db.get_or_create_meta("bbb").unwrap().unwrap();
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

            let m1 = db.get_or_create_meta("aaa").unwrap().unwrap();
            assert_eq!(1, m1.id);
            let m2 = db.get_or_create_meta("bbb").unwrap().unwrap();
            assert_eq!(2, m2.id);
            let m3 = db.get_meta("ccc").unwrap();
            assert!(m3.is_none());

            let m4 = db.get_or_create_meta("ccc").unwrap().unwrap();
            assert_eq!(3, m4.id);

            dump_database_meta(&db);
        }
    }
}
