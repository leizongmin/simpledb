use cedar::encoding::KeyType;
use common::*;

mod common;

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
        db.for_each_key(|_, _| {
            counter += 1;
            true
        });
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

        let vec = db.set_items(key).unwrap();
        assert_eq!(2, vec.len());
        assert_eq!("aaa".as_bytes(), vec.get(0).unwrap().as_ref());
        assert_eq!("bbb".as_bytes(), vec.get(1).unwrap().as_ref());

        assert_eq!(true, db.set_delete(key, "aaa".as_bytes()).unwrap());
        assert_eq!(false, db.set_delete(key, "aaa".as_bytes()).unwrap());
        assert_eq!(1, db.set_count(key).unwrap());
        assert_eq!(false, db.set_is_member(key, "aaa".as_bytes()).unwrap());
        assert_eq!(true, db.set_is_member(key, "bbb".as_bytes()).unwrap());
    }
}