use bytes::{Buf, BufMut, BytesMut};

lazy_static! {
    pub static ref PREFIX_META: &'static [u8] = b"m";
    pub static ref PREFIX_DATA: &'static [u8] = b"d";
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

#[derive(Debug, Clone)]
pub struct Meta {
    pub id: u64,
    pub count: u64,
    pub extra: Option<Vec<u8>>,
}

impl Meta {
    pub fn new(id: u64) -> Meta {
        Meta { id, count: 0, extra: None }
    }

    pub fn from_bytes(input: &[u8]) -> Meta {
        let mut buf = input;
        let id = buf.get_u64();
        let count = buf.get_u64();
        let extra = if buf.remaining() > 0 { Some(buf.bytes().to_vec()) } else { None };
        Meta { id, count, extra }
    }

    pub fn get_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(16);
        buf.put_u64(self.id);
        buf.put_u64(self.count);
        if let Some(b) = &self.extra { buf.put_slice(b) }
        buf
    }

    pub fn decode_list_extra(&self) -> Option<(i64, i64)> {
        if let Some(b) = &self.extra {
            let mut buf = b.as_slice();
            let left = buf.get_i64();
            let right = buf.get_i64();
            Some((left, right))
        } else { None }
    }
    pub fn encode_list_extra(&mut self, left: i64, right: i64) {
        let mut buf = BytesMut::with_capacity(16);
        buf.put_i64(left);
        buf.put_i64(right);
        self.extra = Some(buf.to_vec());
    }
}
