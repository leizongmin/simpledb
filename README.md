# simpledb

![Rust](https://github.com/leizongmin/simpledb/workflows/Rust/badge.svg)
[![API](https://docs.rs/simpledb/badge.svg)](https://docs.rs/simpledb)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.45+-lightgray.svg)](https://github.com/leizongmin/simpledb#rust-version-requirements)

NoSQL embedded database on top of RocksDB.

Documents: https://docs.rs/simpledb

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
simpledb = "0.1"
```

Example:

```rust
use simpledb::Database;

fn main() {
    // open a database
    let db = Database::open("./target/path/to/database").unwrap();
    // left push a value to a list
    db.list_left_push("key", "value".as_bytes()).unwrap();
}
```

## Data Types and Methods

### Map

Store field/value pairs, includes the following methods with `map_` prefix: `get`, `put`, `delete`, `count`, `for_each`, `items`.

### Set

Store unique values, includes the following methods with `set_` prefix: `add`, `is_member`, `delete`, `count`, `for_each`, `items`.

### List

Store ordered values, includes the following methods with `list_` prefix: `left_push`, `left_pop`, `right_push`, `right_pop`, `count`, `for_each`, `items`.

### Sorted List

Store sorted score/value pairs, includes the following methods with `sorted_list_` prefix: `add`, `left_pop`, `right_pop`, `count`, `for_each`, `items`.

### Sorted Set

store sorted score/value pairs, includes the following methods with `sorted_set_` prefix: `add`, `is_member`, `delete`, `left`, `right`, `for_each`, `items`.

---

**Notes: the difference between `sorted list` and `sorted set` is `list` allow the same members, `set` does not allow
the same members.**

## Benchmark

Example codes from `benchmark` directory.

- rustc 1.51.0-nightly (44e3daf5e 2020-12-31)
- macOS Big Sur 11.1
- Intel(R) Core(TM) i7-6820HQ CPU @ 2.70GHz, MacBook Pro (15-inch, 2016)

| method                | write | op/s    |
| --------------------- | ----- | ------- |
| map_get               |       | 355,871 |
| map_count             |       | 735,294 |
| map_put               | yes   | 62,539  |
| map_delete            | yes   | 62,656  |
| set_count             |       | 813,008 |
| set_is_member         |       | 380,228 |
| set_add               | yes   | 64,724  |
| set_delete            | yes   | 61,349  |
| list_count            |       | 666,666 |
| list_left_push        | yes   | 68,775  |
| list_left_pop         | yes   | 60,277  |
| list_right_push       | yes   | 64,641  |
| list_right_pop        | yes   | 56,433  |
| sorted_list_count     |       | 588,235 |
| sorted_list_add       | yes   | 68,493  |
| sorted_list_left_pop  | yes   | 14,880  |
| sorted_list_right_pop | yes   | 7,923   |
| sorted_set_is_member  |       | 285,714 |
| sorted_set_count      |       | 500,000 |
| sorted_set_add        | yes   | 47,393  |
| sorted_set_left       | yes   | 16,181  |
| sorted_set_right      | yes   | 9,082   |
| sorted_set_delete     | yes   | 19,920  |

## License

```text
MIT License

Copyright (c) 2020-2021 Zongmin Lei <leizongmin@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
