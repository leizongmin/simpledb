# simpledb

![Rust](https://github.com/leizongmin/simpledb/workflows/Rust/badge.svg)
[![API](https://docs.rs/simpledb/badge.svg)](https://docs.rs/simpledb)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.45+-lightgray.svg)](https://github.com/leizongmin/simpledb#rust-version-requirements)

NoSQL embedded database on top of RocksDB.

API Documents: https://docs.rs/simpledb

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
simpledb = "0.1.5"
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

Store key/value pairs, includes the following methods with `map_` prefix: `get`, `put`, `delete`, `count`, `for_each`, `items`.

### Set

Store unique values, includes the following methods with `set_` prefix: `add`, `is_member`, `delete`, `count`, `for_each`, `items`.

### List

Store ordered values, includes the following methods with `list_` prefix: `left_push`, `left_pop`, `right_push`, `right_pop`, `count`, `for_each`, `items`.

### Sorted List

Store sorted score/value pairs, may including multiple pairs, includes the following methods with `sorted_list_` prefix: `add`, `left_pop`, `right_pop`, `count`, `for_each`, `items`.

### Sorted Set

Store sorted unique score/value pairs, includes the following methods with `sorted_set_` prefix: `add`, `is_member`, `delete`, `left`, `right`, `for_each`, `items`.

## Benchmark

Example codes from `benchmark` directory.

- rustc 1.67.1 (d5a82bbd2 2023-02-07)
- Ubuntu 18.04
- Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz x 16

| method                | write | op/s      |
| --------------------- | ----- | --------- |
| map_get               |       | 662,251   |
| map_count             |       | 1,369,863 |
| map_put               |   Y   | 105,152   |
| map_delete            |   Y   | 99,900    |
| set_count             |       | 1,639,344 |
| set_is_member         |       | 746,268   |
| set_add               |   Y   | 109,409   |
| set_delete            |   Y   | 105,152   |
| list_count            |       | 1,136,363 |
| list_left_push        |   Y   | 116,414   |
| list_left_pop         |   Y   | 96,339    |
| list_right_push       |   Y   | 115,207   |
| list_right_pop        |   Y   | 98,522    |
| sorted_list_count     |       | 1,666,666 |
| sorted_list_add       |   Y   | 121,951   |
| sorted_list_left_pop  |   Y   | 20,080    |
| sorted_list_right_pop |   Y   | 9,891     |
| sorted_set_is_member  |       | 666,666   |
| sorted_set_count      |       | 1,428,571 |
| sorted_set_add        |   Y   | 83,333    |
| sorted_set_left       |   Y   | 40,160    |
| sorted_set_right      |   Y   | 13,793    |
| sorted_set_delete     |   Y   | 27,548    |

## License

```text
MIT License

Copyright (c) 2020-2023 LEI Zongmin <leizongmin@gmail.com>

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
