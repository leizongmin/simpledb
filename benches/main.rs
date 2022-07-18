use std::{
    env::current_exe,
    fs::{create_dir_all, remove_dir_all},
};

use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
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

fn criterion_benchmark(c: &mut Criterion) {
    let db = open().unwrap();
    let mut rng = rand::thread_rng();
    let table = b"test";

    macro_rules! run {
        ($name:ident $func:expr) => {
            c.bench_function(stringify!($name), $func);
        };
    }

    run!(
        map_put | b | {
            b.iter(|| {
                let key: [u8; 32] = rng.gen();
                let val: [u8; 32] = rng.gen();
                db.map_put(table, key, val)?;
                Ok::<_, anyhow::Error>(())
            })
        }
    );

    run!(
        map_get | b | {
            b.iter(|| {
                let key: [u8; 32] = rng.gen();
                db.map_get(table, key)?;
                Ok::<_, anyhow::Error>(())
            })
        }
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
