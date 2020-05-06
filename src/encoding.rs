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

pub fn encode_data_key_map_field(key_id: u64, field: &str) -> BytesMut {
    let field = field.as_bytes();
    let mut buf = BytesMut::with_capacity(9 + field.len());
    buf.put_slice(*PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_slice(field);
    buf
}

pub fn decode_data_key_map_field(key: &[u8]) -> String {
    String::from_utf8(key[9..].to_vec()).unwrap()
}

pub fn encode_data_key_set_value(key_id: u64, value: &[u8]) -> BytesMut {
    let mut buf = BytesMut::with_capacity(9 + value.len());
    buf.put_slice(*PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_slice(value);
    buf
}

pub fn decode_data_key_set_value(key: &[u8]) -> &[u8] {
    key[9..].as_ref()
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

    pub fn decode_list_extra(&self) -> Option<(i64, i64)> {
        if let Some(b) = &self.extra {
            let mut buf = b.as_slice();
            let left = buf.get_i64();
            let right = buf.get_i64();
            Some((left, right))
        } else {
            None
        }
    }
    pub fn encode_list_extra(&mut self, left: i64, right: i64) {
        let mut buf = BytesMut::with_capacity(16);
        buf.put_i64(left);
        buf.put_i64(right);
        self.extra = Some(buf.to_vec());
    }
}
