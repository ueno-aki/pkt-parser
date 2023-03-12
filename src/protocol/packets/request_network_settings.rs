use anyhow::Result;

use crate::protodef::reader::Read;

#[derive(Debug)]
pub struct RequestNetworkSetting {
    pub client_protocol: i32,
}

impl RequestNetworkSetting {
    pub fn from_vec_u8(buf: &[u8], offset: u64) -> Result<Self> {
        let client_protocol = buf.read_i32(offset);
        Ok(RequestNetworkSetting { client_protocol })
    }
}
