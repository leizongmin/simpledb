use std::env::temp_dir;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use cedar::database::Database;

#[allow(dead_code)]
pub fn benchmark_test_case<F>(title: &str, count: usize, mut f: F)
where
    F: FnMut(usize),
{
    // println!("start {}...", title);
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    f(count);
    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let spent = end - start;
    let qps = count as f64 / (spent as f64 / 1000 as f64);
    println!(
        "{:30}\tcount={}\tspent={}ms\tqps={}",
        title, count, spent, qps as i64
    );
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn open_database() -> Database {
    let path = get_random_database_path();
    Database::destroy(&path).unwrap();
    open_database_with_path(&path)
}

#[allow(dead_code)]
pub fn open_database_with_path(path: &str) -> Database {
    let db = Database::open(path).unwrap();
    println!("open database: {}", db.path);
    db
}

#[macro_export]
macro_rules! open_database {
    () => {
        crate::common::open_database_with_path(crate::common::get_random_database_path().as_str())
    };
}
