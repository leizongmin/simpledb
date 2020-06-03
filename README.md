# simpledb

![Rust](https://github.com/leizongmin/simpledb/workflows/Rust/badge.svg)
[![API](https://docs.rs/simpledb/badge.svg)](https://docs.rs/simpledb)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.45+-lightgray.svg)](https://github.com/leizongmin/simpledb#rust-version-requirements)

NoSQL embedded database on top of RocksDB.

Documents: https://docs.rs/simpledb

## Usage

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

Supported data type:
- **map**: Store field/value pairs, includes the following operations with `map_` prefix: `get`, `put`, `delete`, `count`, `for_each`, `items`.
- **set**: Store unique values, includes the following operations with `set_` prefix: `add`, `is_member`, `delete`, `count`, `for_each`, `items`.
- **list**: Store ordered values, includes the following operations with `list_` prefix: `left_push`, `left_pop`, `right_push`, `right_pop`, `count`, `for_each`, `items`.
- **sorted list**: Store sorted score/value pairs, includes the following operations with `sorted_list_` prefix: `add`, `left_pop`, `right_pop`, `count`, `for_each`, `items`.


## Changelog

- v0.1.1:
  - feat: add `map_for_each_with_prefix` & `map_items_with_prefix` operations.


## License

```text
MIT License

Copyright (c) 2020 Zongmin Lei <leizongmin@gmail.com>

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