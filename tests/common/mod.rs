use std::env::temp_dir;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use cedar::database::Database;

pub fn benchmark_test_case<F>(title: &str, mut f: F)
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

pub fn get_random_database_path() -> String {
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

pub fn open_database() -> Database {
    let path = get_random_database_path();
    Database::destroy(&path).unwrap();
    open_database_with_path(&path)
}

pub fn open_database_with_path(path: &str) -> Database {
    let db = Database::open(path).unwrap();
    println!("open database: {}", db.path);
    db
}

pub fn dump_database_meta(db: &Database) {
    println!("dump_database_meta:");
    db.for_each_key(|k, m| {
        println!("key: {:?}\t value: {:?}", k, m);
        true
    });
}

pub fn dump_database_data(db: &Database, key: &str) {
    println!("dump_database_data:");
    db.for_each_data(key, |k, m| {
        println!("key: {:?}\t value: {:?}", k, m);
        true
    }).unwrap();
}

pub fn vec_to_str(vec: Vec<u8>) -> String {
    String::from_utf8(vec).unwrap()
}
