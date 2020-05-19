use cedar::encoding::{get_score_bytes, get_score_from_bytes, KeyType};
use common::*;

pub mod common;

#[test]
fn test_get_or_save_meta() {
    let path = get_random_database_path();
    {
        let mut db = open_database_with_path(&path);
        let m1 = db.get_or_create_meta("aaa", KeyType::Map).unwrap();
        assert_eq!(1, m1.id);
        let m2 = db.get_or_create_meta("bbb", KeyType::List).unwrap();
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

        let m1 = db.get_or_create_meta("aaa", KeyType::Map).unwrap();
        assert_eq!(1, m1.id);
        assert_eq!(KeyType::Map, m1.key_type);
        let m2 = db.get_or_create_meta("bbb", KeyType::List).unwrap();
        assert_eq!(2, m2.id);
        assert_eq!(KeyType::List, m2.key_type);
        let m3 = db.get_meta("ccc").unwrap();
        assert!(m3.is_none());

        let m4 = db.get_or_create_meta("ccc", KeyType::Set).unwrap();
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
        db.for_each_key(|_, _| {
            counter += 1;
            true
        }).unwrap();
        assert_eq!(0, counter);
    }
}

#[test]
fn test_set() {
    let path = get_random_database_path();
    {
        let mut db = open_database_with_path(&path);
        let key = "hello";

        assert_eq!(false, db.set_is_member(key, "aaa".as_bytes()).unwrap());
        assert_eq!(0, db.set_count(key).unwrap());

        assert_eq!(true, db.set_add(key, "aaa".as_bytes()).unwrap());
        assert_eq!(true, db.set_add(key, "bbb".as_bytes()).unwrap());
        assert_eq!(false, db.set_add(key, "bbb".as_bytes()).unwrap());
        assert_eq!(2, db.set_count(key).unwrap());
        assert_eq!(true, db.set_is_member(key, "aaa".as_bytes()).unwrap());
        assert_eq!(true, db.set_is_member(key, "bbb".as_bytes()).unwrap());
        dump_database_meta(&db);
        dump_database_data(&db, key);

        let vec = db.set_items(key).unwrap();
        assert_eq!(2, vec.len());
        assert_eq!("aaa".as_bytes(), vec.get(0).unwrap().as_ref());
        assert_eq!("bbb".as_bytes(), vec.get(1).unwrap().as_ref());

        assert_eq!(true, db.set_delete(key, "aaa".as_bytes()).unwrap());
        assert_eq!(false, db.set_delete(key, "aaa".as_bytes()).unwrap());
        assert_eq!(1, db.set_count(key).unwrap());
        assert_eq!(false, db.set_is_member(key, "aaa".as_bytes()).unwrap());
        assert_eq!(true, db.set_is_member(key, "bbb".as_bytes()).unwrap());
        dump_database_meta(&db);
        dump_database_data(&db, key);
    }
}

#[test]
fn test_list() {
    let path = get_random_database_path();
    {
        let mut db = open_database_with_path(&path);
        let key = "hello";

        assert_eq!(0, db.list_count(key).unwrap());
        assert_eq!(None, db.list_left_pop(key).unwrap());
        assert_eq!(None, db.list_right_pop(key).unwrap());

        assert_eq!(1, db.list_left_push(key, "a".as_bytes()).unwrap());
        assert_eq!(2, db.list_left_push(key, "b".as_bytes()).unwrap());
        assert_eq!(3, db.list_left_push(key, "c".as_bytes()).unwrap());
        assert_eq!(4, db.list_right_push(key, "d".as_bytes()).unwrap());
        assert_eq!(5, db.list_right_push(key, "e".as_bytes()).unwrap());
        assert_eq!(6, db.list_right_push(key, "f".as_bytes()).unwrap());
        assert_eq!(7, db.list_right_push(key, "g".as_bytes()).unwrap());
        assert_eq!(7, db.list_count(key).unwrap());
        dump_database_meta(&db);
        dump_database_data(&db, key);

        let vec = db.list_items(key).unwrap();
        assert_eq!(7, vec.len());
        assert_eq!(
            "cbadefg",
            String::from_utf8(vec.iter().flat_map(|v| v.to_vec()).collect::<Vec<u8>>()).unwrap()
        );

        assert_eq!(
            "c".as_bytes(),
            db.list_left_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(
            "g".as_bytes(),
            db.list_right_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(
            "f".as_bytes(),
            db.list_right_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(4, db.list_count(key).unwrap());
        dump_database_meta(&db);
        dump_database_data(&db, key);

        assert_eq!(5, db.list_right_push(key, "x".as_bytes()).unwrap());
        assert_eq!(6, db.list_right_push(key, "y".as_bytes()).unwrap());
        assert_eq!(7, db.list_left_push(key, "z".as_bytes()).unwrap());
        let vec = db.list_items(key).unwrap();
        assert_eq!(7, vec.len());
        assert_eq!(
            "zbadexy",
            String::from_utf8(vec.iter().flat_map(|v| v.to_vec()).collect::<Vec<u8>>()).unwrap()
        );
        dump_database_meta(&db);
        dump_database_data(&db, key);

        assert_eq!(
            "z".as_bytes(),
            db.list_left_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(
            "b".as_bytes(),
            db.list_left_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(
            "a".as_bytes(),
            db.list_left_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(
            "d".as_bytes(),
            db.list_left_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(
            "e".as_bytes(),
            db.list_left_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(
            "x".as_bytes(),
            db.list_left_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(
            "y".as_bytes(),
            db.list_left_pop(key).unwrap().unwrap().as_ref()
        );
        assert_eq!(None, db.list_left_pop(key).unwrap());
        assert_eq!(None, db.list_right_pop(key).unwrap());
        assert_eq!(0, db.list_count(key).unwrap());
        dump_database_meta(&db);
        dump_database_data(&db, key);
    }
}

#[test]
fn test_sorted_list() {
    let path = get_random_database_path();
    {
        let mut db = open_database_with_path(&path);
        let key = "hello";

        assert_eq!(0, db.sorted_list_count(key).unwrap());

        assert_eq!(
            1,
            db.sorted_list_add(key, get_score_bytes(123).as_slice(), "a".as_bytes())
                .unwrap()
        );
        assert_eq!(
            2,
            db.sorted_list_add(key, get_score_bytes(120).as_slice(), "b".as_bytes())
                .unwrap()
        );
        assert_eq!(
            3,
            db.sorted_list_add(key, get_score_bytes(0).as_slice(), "c".as_bytes())
                .unwrap()
        );
        assert_eq!(
            4,
            db.sorted_list_add(key, get_score_bytes(120).as_slice(), "d".as_bytes())
                .unwrap()
        );
        assert_eq!(
            5,
            db.sorted_list_add(key, get_score_bytes(-5).as_slice(), "e".as_bytes())
                .unwrap()
        );
        assert_eq!(
            6,
            db.sorted_list_add(key, get_score_bytes(-10).as_slice(), "f".as_bytes())
                .unwrap()
        );
        assert_eq!(6, db.sorted_list_count(key).unwrap());
        dump_database_meta(&db);
        dump_database_data(&db, key);

        let vec = db.sorted_list_items(key).unwrap();
        assert_eq!(6, vec.len());
        let scores: Vec<i32> = vec.iter().map(|(s, _)| get_score_from_bytes(s)).collect();
        assert_eq!(vec![-10, -5, 0, 120, 120, 123], scores);
        let values: Vec<String> = vec
            .iter()
            .map(|(_, v)| String::from_utf8(v.to_vec()).unwrap())
            .collect();
        assert_eq!(vec!["f", "e", "c", "b", "d", "a"], values);

        assert_eq!(
            None,
            db.sorted_list_left_pop(key, Some(get_score_bytes(-200).as_slice()))
                .unwrap()
        );
        assert_eq!(
            None,
            db.sorted_list_right_pop(key, Some(get_score_bytes(200).as_slice()))
                .unwrap()
        );

        {
            let (score, value) = db
                .sorted_list_left_pop(key, Some(get_score_bytes(-8).as_slice()))
                .unwrap()
                .unwrap();
            assert_eq!(-10, get_score_from_bytes(score.as_ref()));
            assert_eq!("f", String::from_utf8(value.to_vec()).unwrap());
            assert_eq!(
                None,
                db.sorted_list_left_pop(key, Some(get_score_bytes(-8).as_slice()))
                    .unwrap()
            );
        }
        {
            let (score, value) = db
                .sorted_list_right_pop(key, Some(get_score_bytes(121).as_slice()))
                .unwrap()
                .unwrap();
            assert_eq!(123, get_score_from_bytes(score.as_ref()));
            assert_eq!("a", String::from_utf8(value.to_vec()).unwrap());
            assert_eq!(
                None,
                db.sorted_list_right_pop(key, Some(get_score_bytes(121).as_slice()))
                    .unwrap()
            );
        }
        {
            let (score, value) = db.sorted_list_left_pop(key, None).unwrap().unwrap();
            assert_eq!(-5, get_score_from_bytes(score.as_ref()));
            assert_eq!("e", String::from_utf8(value.to_vec()).unwrap());
        }
        {
            let (score, value) = db.sorted_list_right_pop(key, None).unwrap().unwrap();
            assert_eq!(120, get_score_from_bytes(score.as_ref()));
            assert_eq!("d", String::from_utf8(value.to_vec()).unwrap());
        }
        assert_eq!(2, db.sorted_list_count(key).unwrap());

        let vec = db.sorted_list_items(key).unwrap();
        assert_eq!(2, vec.len());
        let scores: Vec<i32> = vec.iter().map(|(s, _)| get_score_from_bytes(s)).collect();
        assert_eq!(vec![0, 120], scores);
        let values: Vec<String> = vec
            .iter()
            .map(|(_, v)| String::from_utf8(v.to_vec()).unwrap())
            .collect();
        assert_eq!(vec!["c", "b"], values);

        dump_database_meta(&db);
        dump_database_data(&db, key);
    }
}

#[test]
fn test_delete_all() {
    let path = get_random_database_path();
    let mut db = open_database_with_path(&path);
    let count = 10;
    let key1 = "hello";
    let key2 = "world";
    for i in 1..=count {
        db.map_put(
            key1,
            format!("key_{}", i).as_str(),
            get_score_bytes(i).as_slice(),
        )
        .unwrap();
    }
    for i in 1..=count {
        db.list_right_push(key2, get_score_bytes(i).as_slice())
            .unwrap();
    }

    dump_database_meta(&db);
    dump_database_data(&db, key1);
    dump_database_data(&db, key2);

    db.delete_all(key1).unwrap();
    dump_database_meta(&db);
    dump_database_data(&db, key1);
    dump_database_data(&db, key2);
    assert_eq!(0, db.get_count(key1).unwrap());
    assert_eq!(count, db.get_count(key2).unwrap());

    db.delete_all(key2).unwrap();
    dump_database_meta(&db);
    dump_database_data(&db, key1);
    dump_database_data(&db, key2);
    assert_eq!(0, db.get_count(key1).unwrap());
    assert_eq!(0, db.get_count(key2).unwrap());
}
