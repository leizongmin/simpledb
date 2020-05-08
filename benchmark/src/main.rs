use cedar::encoding::get_score_bytes;

pub mod common;

fn main() {
    test_map();
    test_sorted_list();
}

fn test_map() {
    let mut db = common::open_database_with_path(common::get_random_database_path().as_str());
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

fn test_sorted_list() {
    let mut db = common::open_database_with_path(common::get_random_database_path().as_str());
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
