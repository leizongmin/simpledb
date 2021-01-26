use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use simpledb::encoding::get_score_bytes;

#[macro_use]
pub mod common;

fn main() {
    test_multi_threading();
    test_multi_threading2();
    test_map();
    test_set();
    test_list();
    test_sorted_list();
    test_sorted_set();
}

fn test_multi_threading() {
    let db = open_database!();
    let db = Arc::new(Mutex::new(db));
    for _ in 0..3 {
        let db = db.clone();
        thread::spawn(move || {
            let ret = db.lock().unwrap().get_meta("a").unwrap();
            println!("{:?}", ret);
        });
    }
    thread::sleep(Duration::from_millis(50));
}

fn test_multi_threading2() {
    let db = open_database!();
    let db = Arc::new(db);
    let threads: Vec<_> = (0..10)
        .map(|_| {
            let db = db.clone();
            thread::spawn(move || {
                let mut counter = 0;
                for v in 0..1000 {
                    db.get_meta("a").unwrap();
                    db.map_put("a", "a", "aaa".as_bytes()).unwrap();
                    counter += 1;
                }
                (counter, std::time::SystemTime::now())
            })
        })
        .collect();
    let results: Vec<_> = threads.into_iter().map(|t| t.join().unwrap()).collect();
    println!("{:?}", results);
}

fn test_map() {
    let db = open_database!();
    let key = "hello_map";
    let value = "hello, world".as_bytes();
    let count = 10_0000;
    let fields: Vec<String> = (0..count).map(|i| format!("field_{}", i)).collect();

    common::benchmark_test_case("map_put", count, |_| {
        for f in &fields {
            db.map_put(key, f, value).unwrap();
        }
    });
    common::benchmark_test_case("map_get", count, |_| {
        for f in &fields {
            db.map_get(key, f).unwrap();
        }
    });
    common::benchmark_test_case("map_count", count, |count| {
        for _ in 0..count {
            db.map_count(key).unwrap();
        }
    });
    common::benchmark_test_case("map_delete", count, |_| {
        for f in &fields {
            db.map_delete(key, f).unwrap();
        }
    });
}

fn test_set() {
    let db = open_database!();
    let key = "hello_set";
    let count = 10_0000;
    let values: Vec<String> = (0..count).map(|i| format!("field_{}", i)).collect();

    common::benchmark_test_case("set_add", count, |_| {
        for v in &values {
            db.set_add(key, v.as_bytes()).unwrap();
        }
    });
    common::benchmark_test_case("set_count", count, |_| {
        for _ in &values {
            db.set_count(key).unwrap();
        }
    });
    common::benchmark_test_case("set_is_member", count, |_| {
        for v in &values {
            db.set_is_member(key, v.as_bytes()).unwrap();
        }
    });
    common::benchmark_test_case("set_delete", count, |_| {
        for v in &values {
            db.set_delete(key, v.as_bytes()).unwrap();
        }
    });
}

fn test_list() {
    let db = open_database!();
    let key = "hello_list";
    let count = 10_0000;
    let values: Vec<String> = (0..count).map(|i| format!("field_{}", i)).collect();

    common::benchmark_test_case("list_left_push", count, |_| {
        for v in &values {
            db.list_left_push(key, v.as_bytes()).unwrap();
        }
    });
    common::benchmark_test_case("list_count", count, |_| {
        for _ in &values {
            db.list_count(key).unwrap();
        }
    });
    common::benchmark_test_case("list_left_pop", count, |_| {
        for _ in &values {
            db.list_left_pop(key).unwrap();
        }
    });
    common::benchmark_test_case("list_right_push", count, |_| {
        for v in &values {
            db.list_right_push(key, v.as_bytes()).unwrap();
        }
    });
    common::benchmark_test_case("list_count", count, |_| {
        for _ in &values {
            db.list_count(key).unwrap();
        }
    });
    common::benchmark_test_case("list_right_pop", count, |_| {
        for _ in &values {
            db.list_right_pop(key).unwrap();
        }
    });
}

fn test_sorted_list() {
    let db = open_database!();
    let key = "hello_sorted_list";
    let count = 1_0000;
    let items: Vec<(Vec<u8>, String)> = (0..count)
        .map(|i| (get_score_bytes(i as i64), format!("field_{}", i)))
        .collect();

    common::benchmark_test_case("sorted_list_add", count, |_| {
        for (score, value) in &items {
            db.sorted_list_add(key, score.as_slice(), value.as_bytes())
                .unwrap();
        }
    });
    common::benchmark_test_case("sorted_list_count", count, |count| {
        for _ in 0..count {
            db.sorted_list_count(key).unwrap();
        }
    });
    common::benchmark_test_case("sorted_list_left_pop", count, |_| {
        for (score, _) in &items {
            db.sorted_list_left_pop(key, Some(score.as_slice()))
                .unwrap();
        }
    });
    common::benchmark_test_case("sorted_list_add", count, |_| {
        for (score, value) in &items {
            db.sorted_list_add(key, score.as_slice(), value.as_bytes())
                .unwrap();
        }
    });
    common::benchmark_test_case("sorted_list_right_pop", count, |_| {
        for (score, _) in &items {
            db.sorted_list_right_pop(key, Some(score.as_slice()))
                .unwrap();
        }
    });
}

fn test_sorted_set() {
    let db = open_database!();
    let key = "hello_sorted_set";
    let count = 1_0000;
    let items: Vec<(Vec<u8>, String)> = (0..count)
        .map(|i| (get_score_bytes(i as i64), format!("field_{}", i)))
        .collect();

    common::benchmark_test_case("sorted_set_add", count, |_| {
        for (score, value) in &items {
            db.sorted_set_add(key, score.as_slice(), value.as_bytes())
                .unwrap();
        }
    });
    common::benchmark_test_case("sorted_set_is_member", count, |_| {
        for (score, value) in &items {
            db.sorted_set_is_member(key, value.as_bytes()).unwrap();
        }
    });
    common::benchmark_test_case("sorted_set_count", count, |count| {
        for _ in 0..count {
            db.sorted_set_count(key).unwrap();
        }
    });
    common::benchmark_test_case("sorted_set_left", count, |_| {
        for (score, _) in &items {
            db.sorted_set_left(key, Some(score.as_slice()), 100)
                .unwrap();
        }
    });
    common::benchmark_test_case("sorted_set_right", count, |_| {
        for (score, _) in &items {
            db.sorted_set_right(key, Some(score.as_slice()), 100)
                .unwrap();
        }
    });
    common::benchmark_test_case("sorted_set_delete", count, |_| {
        for (score, value) in &items {
            db.sorted_set_delete(key, value.as_bytes()).unwrap();
        }
    });
}
