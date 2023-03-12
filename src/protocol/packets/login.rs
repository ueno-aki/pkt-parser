use anyhow::Result;

use crate::protodef::reader::Read;

#[derive(Debug)]
pub struct Login {
    pub protocol_version: i32,
    pub identity: String,
    pub client: String,
}

impl Login {
    pub fn from_vec_u8(buf: &[u8], offset: u64) -> Result<Self> {
        let mut cursor = offset;
        let protocol_version = buf.read_i32(cursor);
        cursor += 4;
        let (_payload, payload_size) = buf.read_varint(cursor)?;
        cursor += payload_size;
        let (identity, identity_size) = buf.read_little_string(cursor)?;
        cursor += identity_size;
        let (client, _client_size) = buf.read_little_string(cursor)?;
        Ok(Login {
            protocol_version,
            identity,
            client,
        })
    }
}
