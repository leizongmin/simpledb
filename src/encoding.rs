use bytes::{Buf, BufMut, BytesMut};

lazy_static! {
    pub static ref PREFIX_META: &'static [u8] = b"m";
    pub static ref PREFIX_DATA: &'static [u8] = b"d";
    pub static ref FILL_EMPTY_DATA: &'static [u8] = b"";
}

pub fn has_prefix(prefix: &[u8], key: &[u8]) -> bool {
    prefix.iter().zip(key).take_while(|(x, y)| x == y).count() == prefix.len()
}

pub fn encode_meta_key(key: &str) -> BytesMut {
    let mut buf = BytesMut::with_capacity(9);
    buf.put_slice(*PREFIX_META);
    buf.put_slice(key.as_bytes());
    buf
}

pub fn decode_meta_key(key: &[u8]) -> String {
    String::from_utf8(key[1..].to_vec()).unwrap()
}

pub fn encode_data_key(key_id: u64) -> BytesMut {
    let mut buf = BytesMut::with_capacity(9);
    buf.put_slice(*PREFIX_DATA);
    buf.put_u64(key_id);
    buf
}

pub fn encode_data_key_map_item(key_id: u64, field: &str) -> BytesMut {
    let field = field.as_bytes();
    let mut buf = BytesMut::with_capacity(9 + field.len());
    buf.put_slice(*PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_slice(field);
    buf
}

pub fn decode_data_key_map_item(key: &[u8]) -> String {
    String::from_utf8(key[9..].to_vec()).unwrap()
}

pub fn encode_data_key_set_item(key_id: u64, value: &[u8]) -> BytesMut {
    let mut buf = BytesMut::with_capacity(9 + value.len());
    buf.put_slice(*PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_slice(value);
    buf
}

pub fn decode_data_key_set_item(key: &[u8]) -> &[u8] {
    key[9..].as_ref()
}

pub fn encode_data_key_list_item(key_id: u64, position: i64) -> BytesMut {
    let mut buf = BytesMut::with_capacity(18);
    buf.put_slice(*PREFIX_DATA);
    buf.put_u64(key_id);
    if position >= 0 {
        buf.put_slice(b">");
    } else {
        buf.put_slice(b"<");
    }
    buf.put_i64(position);
    buf
}

pub fn encode_data_key_sorted_list_item(key_id: u64, score: &[u8], sequence: u64) -> BytesMut {
    let mut buf = BytesMut::with_capacity(17 + score.len());
    buf.put_slice(*PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_slice(score);
    buf.put_u64(sequence);
    buf
}

pub fn decode_data_key_sorted_list_item(key: &[u8]) -> &[u8] {
    key[9..key.len() - 8].as_ref()
}

pub fn compare_score_bytes(a: &[u8], b: &[u8]) -> i32 {
    if a > b { 1 } else if a < b { -1 } else { 0 }
}

pub trait BytesComparableScore {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(b: &[u8]) -> Self where Self: Sized;
}

impl BytesComparableScore for i64 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(9);
        if *self >= 0 {
            buf.put_slice(b">");
        } else {
            buf.put_slice(b"<");
        }
        buf.put_i64(*self);
        buf.to_vec()
    }

    fn from_bytes(b: &[u8]) -> Self where Self: Sized {
        b[1..].as_ref().to_bytes().get_i64()
    }
}

impl BytesComparableScore for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(5);
        if *self >= 0 {
            buf.put_slice(b">");
        } else {
            buf.put_slice(b"<");
        }
        buf.put_i32(*self);
        buf.to_vec()
    }

    fn from_bytes(b: &[u8]) -> Self where Self: Sized {
        b[1..].as_ref().to_bytes().get_i32()
    }
}

impl BytesComparableScore for f64 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(9);
        if *self >= 0.0 {
            buf.put_slice(b">");
        } else {
            buf.put_slice(b"<");
        }
        buf.put_f64(*self);
        buf.to_vec()
    }

    fn from_bytes(b: &[u8]) -> Self where Self: Sized {
        b[1..].as_ref().to_bytes().get_f64()
    }
}

impl BytesComparableScore for f32 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(5);
        if *self >= 0.0 {
            buf.put_slice(b">");
        } else {
            buf.put_slice(b"<");
        }
        buf.put_f32(*self);
        buf.to_vec()
    }

    fn from_bytes(b: &[u8]) -> Self where Self: Sized {
        b[1..].as_ref().to_bytes().get_f32()
    }
}

pub fn get_score_bytes<T>(score: T) -> Vec<u8> where T: BytesComparableScore {
    score.to_bytes()
}

pub fn get_score_from_bytes<T>(b: &[u8]) -> T where T: BytesComparableScore {
    BytesComparableScore::from_bytes(b)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeyType {
    Map,
    List,
    SortedList,
    Set,
}

impl KeyType {
    pub fn from_u8(c: u8) -> Option<KeyType> {
        match c {
            1 => Some(KeyType::Map),
            2 => Some(KeyType::List),
            3 => Some(KeyType::SortedList),
            4 => Some(KeyType::Set),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            KeyType::Map => 1,
            KeyType::List => 2,
            KeyType::SortedList => 3,
            KeyType::Set => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyMeta {
    pub id: u64,
    pub key_type: KeyType,
    pub count: u64,
    pub extra: Option<Vec<u8>>,
}

impl KeyMeta {
    pub fn new(id: u64, key_type: KeyType) -> KeyMeta {
        KeyMeta {
            id,
            count: 0,
            key_type,
            extra: None,
        }
    }

    pub fn from_bytes(input: &[u8]) -> KeyMeta {
        let mut buf = input;
        let id = buf.get_u64();
        let key_type = KeyType::from_u8(buf.get_u8()).unwrap_or(KeyType::Map);
        let count = buf.get_u64();
        let extra = if buf.remaining() > 0 {
            Some(buf.bytes().to_vec())
        } else {
            None
        };
        KeyMeta {
            id,
            key_type,
            count,
            extra,
        }
    }

    pub fn get_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(16);
        buf.put_u64(self.id);
        buf.put_u8(self.key_type.to_u8());
        buf.put_u64(self.count);
        if let Some(b) = &self.extra {
            buf.put_slice(b)
        }
        buf
    }

    pub fn decode_list_extra(&self) -> (i64, i64) {
        if let Some(b) = &self.extra {
            let mut buf = b.as_slice();
            let left = buf.get_i64();
            let right = buf.get_i64();
            (left, right)
        } else {
            (0, 1)
        }
    }
    pub fn encode_list_extra(&mut self, left: i64, right: i64) {
        let mut buf = BytesMut::with_capacity(16);
        buf.put_i64(left);
        buf.put_i64(right);
        self.extra = Some(buf.to_vec());
    }

    pub fn decode_sorted_list_extra(&self) -> (u64, u32, u32) {
        if let Some(b) = &self.extra {
            let mut buf = b.as_slice();
            (buf.get_u64(), buf.get_u32(), buf.get_u32())
        } else {
            (0, 0, 0)
        }
    }

    pub fn encode_sorted_list_extra(&mut self, sequence: u64, left_deleted_count: u32, right_deleted_count: u32) {
        let mut buf = BytesMut::with_capacity(12);
        buf.put_u64(sequence);
        buf.put_u32(left_deleted_count);
        buf.put_u32(right_deleted_count);
        self.extra = Some(buf.to_vec())
    }
}
