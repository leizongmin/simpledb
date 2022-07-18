use std::string::FromUtf8Error;

use bytes::{Buf, BufMut, BytesMut};

pub type ScoreVal = (Box<[u8]>, Box<[u8]>);
pub type VecScoreVal = Vec<ScoreVal>;

/// Key prefix for meta data.
pub static PREFIX_META: &[u8] = b"m";
/// Key prefix for row data.
pub static PREFIX_DATA: &[u8] = b"d";
/// Fill data for empty row.
pub static FILL_EMPTY_DATA: &[u8] = b"";

/// Ensure a key name has a specific prefix.
pub fn has_prefix(prefix: &[u8], key: &[u8]) -> bool {
    prefix.iter().zip(key).take_while(|(x, y)| x == y).count() == prefix.len()
}

/// Encode a meta key.
pub fn encode_meta_key(key: impl AsRef<[u8]>) -> BytesMut {
    let mut buf = BytesMut::with_capacity(9);
    buf.put_slice(PREFIX_META);
    buf.put_slice(key.as_ref());
    buf
}

/// Decode meta key.
pub fn decode_meta_key(key: &[u8]) -> Result<String, FromUtf8Error> {
    String::from_utf8(key[1..].to_vec())
}

/// Encode data key
pub fn encode_data_key(key_id: u64) -> BytesMut {
    let mut buf = BytesMut::with_capacity(9);
    buf.put_slice(PREFIX_DATA);
    buf.put_u64(key_id);
    buf
}

/// Encode data key of `map` item.
pub fn encode_data_key_map_item(key_id: u64, field: impl AsRef<[u8]>) -> BytesMut {
    let field = field.as_ref();
    let mut buf = BytesMut::with_capacity(9 + field.len());
    buf.put_slice(PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_slice(field);
    buf
}

/// Decode data key of `map` item.
pub fn decode_data_key_map_item(key: &[u8]) -> Result<String, FromUtf8Error> {
    String::from_utf8(key[9..].to_vec())
}

/// Encode data key of `set` item.
pub fn encode_data_key_set_item(key_id: u64, value: &[u8]) -> BytesMut {
    let mut buf = BytesMut::with_capacity(9 + value.len());
    buf.put_slice(PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_slice(value);
    buf
}

/// Decode data key of `set` item.
pub fn decode_data_key_set_item(key: &[u8]) -> &[u8] {
    key[9..].as_ref()
}

/// Encode data key of `list` item.
pub fn encode_data_key_list_item(key_id: u64, position: i64) -> BytesMut {
    let mut buf = BytesMut::with_capacity(18);
    buf.put_slice(PREFIX_DATA);
    buf.put_u64(key_id);
    if position >= 0 {
        buf.put_slice(b">");
    } else {
        buf.put_slice(b"<");
    }
    buf.put_i64(position);
    buf
}

/// Encode data key of `sorted list` item.
pub fn encode_data_key_sorted_list_item(key_id: u64, score: &[u8], sequence: u64) -> BytesMut {
    let mut buf = BytesMut::with_capacity(17 + score.len());
    buf.put_slice(PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_slice(score);
    buf.put_u64(sequence);
    buf
}

/// Decode data key of `sorted list` item.
pub fn decode_data_key_sorted_list_item(key: &[u8]) -> &[u8] {
    key[9..key.len() - 8].as_ref()
}

/// Encode data key prefix of `sorted set` item
pub fn encode_data_key_sorted_set_prefix(key_id: u64) -> BytesMut {
    let mut buf = BytesMut::with_capacity(10);
    buf.put_slice(PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_u8(1);
    buf
}

/// Encode data key of `sorted set` item
pub fn encode_data_key_sorted_set_item_with_score(
    key_id: u64,
    score: &[u8],
    value: &[u8],
) -> BytesMut {
    let mut buf = BytesMut::with_capacity(10 + score.len() + value.len());
    buf.put_slice(PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_u8(1);
    buf.put_slice(score);
    buf.put_slice(value);
    buf
}

/// Encode data key of `sorted set` item
pub fn encode_data_key_sorted_set_item_without_score(key_id: u64, value: &[u8]) -> BytesMut {
    let mut buf = BytesMut::with_capacity(10 + value.len());
    buf.put_slice(PREFIX_DATA);
    buf.put_u64(key_id);
    buf.put_u8(0);
    buf.put_slice(value);
    buf
}

/// Decode data key for `sorted set` item
pub fn decode_data_key_sorted_set_item_with_score(
    key: &[u8],
    score_len: u8,
) -> (Box<[u8]>, Box<[u8]>) {
    let score_len = score_len as usize;
    let score = &key[10..10 + score_len];
    let value = &key[10 + score_len..];
    (Box::from(score), Box::from(value))
}

/// Compare bytes of two scores. It the first item is greater than the second score, returns 1;
/// If the first item is less than the second item, returns -1; Or else returns 0.
pub fn compare_score_bytes(a: &[u8], b: &[u8]) -> i32 {
    use std::cmp::Ordering;

    match a.cmp(b) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    }
}

pub trait BytesComparableScore {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(b: &[u8]) -> Self
    where
        Self: Sized;
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

    fn from_bytes(b: &[u8]) -> Self
    where
        Self: Sized,
    {
        b[1..].as_ref().get_i64()
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

    fn from_bytes(b: &[u8]) -> Self
    where
        Self: Sized,
    {
        b[1..].as_ref().get_i32()
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

    fn from_bytes(b: &[u8]) -> Self
    where
        Self: Sized,
    {
        b[1..].as_ref().get_f64()
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

    fn from_bytes(b: &[u8]) -> Self
    where
        Self: Sized,
    {
        b[1..].as_ref().get_f32()
    }
}

impl BytesComparableScore for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(5);
        buf.put_slice(b">");
        buf.put_u32(*self);
        buf.to_vec()
    }

    fn from_bytes(b: &[u8]) -> Self
    where
        Self: Sized,
    {
        b[1..].as_ref().get_u32()
    }
}

impl BytesComparableScore for u64 {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(9);
        buf.put_slice(b">");
        buf.put_u64(*self);
        buf.to_vec()
    }

    fn from_bytes(b: &[u8]) -> Self
    where
        Self: Sized,
    {
        b[1..].as_ref().get_u64()
    }
}

pub fn get_score_bytes<T>(score: T) -> Vec<u8>
where
    T: BytesComparableScore,
{
    score.to_bytes()
}

pub fn get_score_from_bytes<T>(b: &[u8]) -> T
where
    T: BytesComparableScore,
{
    BytesComparableScore::from_bytes(b)
}

pub fn get_next_upper_bound(bound: &[u8]) -> Vec<u8> {
    let mut next: Vec<i16> = Vec::from(bound).iter().map(|v| *v as i16).collect();
    if !next.iter().any(|v| *v != 255) {
        next.push(0);
    } else {
        let end = bound.len() - 1;
        next[end] += 1;
        if next[end] > 255 {
            next[end] = 0;
            next[end - 1] += 1;
        }
        for i in 1..end {
            let j = end - i;
            if next[j] > 255 {
                next[j] = 0;
                next[j - 1] += 1;
            }
        }
        if next[0] > 255 {
            next[0] = 255;
            next.push(1);
        }
    }
    next.iter().map(|v| *v as u8).collect()
}

/// Supported data type of this database.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeyType {
    Map,
    List,
    SortedList,
    Set,
    SortedSet,
}

impl KeyType {
    pub fn from_u8(c: u8) -> Option<KeyType> {
        match c {
            1 => Some(KeyType::Map),
            2 => Some(KeyType::List),
            3 => Some(KeyType::SortedList),
            4 => Some(KeyType::Set),
            5 => Some(KeyType::SortedSet),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            KeyType::Map => 1,
            KeyType::List => 2,
            KeyType::SortedList => 3,
            KeyType::Set => 4,
            KeyType::SortedSet => 5,
        }
    }
}

/// Meta data struct.
#[derive(Debug, Clone)]
pub struct KeyMeta {
    /// Auto-increment key ID.
    pub id: u64,
    /// Data type.
    pub key_type: KeyType,
    /// Total items count.
    pub count: u64,
    /// Extra data.
    pub extra: Option<Vec<u8>>,
}

impl KeyMeta {
    /// Create a new `KeyMeta` instance.
    pub fn new(id: u64, key_type: KeyType) -> KeyMeta {
        KeyMeta {
            id,
            count: 0,
            key_type,
            extra: None,
        }
    }

    /// Decode `KeyMeta` from bytes.
    pub fn from_bytes(input: &[u8]) -> KeyMeta {
        let mut buf = input;
        let id = buf.get_u64();
        let key_type = KeyType::from_u8(buf.get_u8()).unwrap_or(KeyType::Map);
        let count = buf.get_u64();
        let extra = if buf.remaining() > 0 {
            Some(buf.to_vec())
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

    /// Get bytes.
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

    /// Decode extra data for `list` data type.
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

    /// Encode extra data for `list` data type.
    pub fn encode_list_extra(&mut self, left: i64, right: i64) {
        let mut buf = BytesMut::with_capacity(16);
        buf.put_i64(left);
        buf.put_i64(right);
        self.extra = Some(buf.to_vec());
    }

    /// Decode extra data for `sorted list` data type.
    /// returns (sequence[u64], left_deleted_count[u32], right_deleted_count[u32])
    pub fn decode_sorted_list_extra(&self) -> (u64, u32, u32) {
        if let Some(b) = &self.extra {
            let mut buf = b.as_slice();
            (buf.get_u64(), buf.get_u32(), buf.get_u32())
        } else {
            (0, 0, 0)
        }
    }

    /// Encode extra data for `sorted list` data type.
    pub fn encode_sorted_list_extra(
        &mut self,
        sequence: u64,
        left_deleted_count: u32,
        right_deleted_count: u32,
    ) {
        let mut buf = BytesMut::with_capacity(12);
        buf.put_u64(sequence);
        buf.put_u32(left_deleted_count);
        buf.put_u32(right_deleted_count);
        self.extra = Some(buf.to_vec())
    }

    /// Decode extra data for `sorted set` data type.
    /// returns (deleted_count[u32], score_len[u8])
    pub fn decode_sorted_set_extra(&self) -> (u32, u8) {
        if let Some(b) = &self.extra {
            let mut buf = b.as_slice();
            (buf.get_u32(), buf.get_u8())
        } else {
            (0, 0)
        }
    }

    /// Encode extra data for `sorted set` data type.
    pub fn encode_sorted_set_extra(&mut self, deleted_count: u32, score_len: u8) {
        let mut buf = BytesMut::with_capacity(12);
        buf.put_u32(deleted_count);
        buf.put_u8(score_len);
        self.extra = Some(buf.to_vec())
    }
}
