use std::{
    env::current_exe,
    fs::{create_dir_all, remove_dir_all},
};

use anyhow::Result;
use simpledb::Database as Db;

fn open() -> Result<Db> {
    let mut path = current_exe()?;
    path.pop();
    let path = path.join("test_db");
    dbg!(&path);
    let _ = remove_dir_all(&path);
    create_dir_all(&path)?;
    Ok(Db::open(path)?)
}

fn main() -> Result<()> {
    let db = open()?;

    let table = "test";
    let key = "a";
    db.map_put(table, key, "b")?;
    dbg!(db.map_get(table, key)?);
    Ok(())
}
