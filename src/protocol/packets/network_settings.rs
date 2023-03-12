use anyhow::Result;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

use crate::protodef::writer::Write;

#[derive(Debug)]
pub struct NetworkSettings {
    pub compression_threshold: u16,
    pub compression_algorithm: CompressionAlgorithmType,
    pub client_throttle: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}

impl NetworkSettings {
    pub fn compose(buffer: &mut Vec<u8>, packet: Self) -> Result<()> {
        buffer.write_u16::<BigEndian>(packet.compression_threshold)?;
        buffer.write_u16::<BigEndian>(packet.compression_algorithm as u16)?;
        buffer.write_bool(packet.client_throttle)?;
        buffer.write_u8(packet.client_throttle_threshold)?;
        buffer.write_f32::<LittleEndian>(packet.client_throttle_scalar)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompressionAlgorithmType {
    Deflate,
    Snappy,
}
