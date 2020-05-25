# simpledb

![Rust](https://github.com/leizongmin/simpledb/workflows/Rust/badge.svg)
[![API](https://docs.rs/simpledb/badge.svg)](https://docs.rs/simpledb)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.45+-lightgray.svg)](https://github.com/leizongmin/simpledb#rust-version-requirements)

NoSQL embedded database on top of RocksDB.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
simpledb = "0.1"
```

example:

```rust
use simpledb::Database;

fn main() {
    let db = Database::open("path").unwrap();
    db.list_left_push("key".as_bytes(), "value".as_bytes()).unwrap()
}
```

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