use std::io::Result;

use crate::protodef::{
    mcbe::{reader::write_little_string, writer::write_string},
    native_types::{
        reader::{read_i32, read_varint},
        writer::{write_bool, write_i32, write_lf32, write_lu16, write_u8},
    },
};

use super::*;

#[derive(Debug)]
pub struct Login {
    pub protocol_version: i32,
    pub identity: String,
    pub client: String,
}

impl Login {
    pub fn new(buf: &[u8], offset: u64) -> Result<Packet> {
        let mut cursor = offset;
        let (protocol_version, protocol_version_size) = read_i32(buf, cursor)?;
        cursor += protocol_version_size;
        let (_payload, payload_size) = read_varint(buf, cursor)?;
        cursor += payload_size;
        let (identity, identity_size) = write_little_string(buf, cursor)?;
        cursor += identity_size;
        let (client, client_size) = write_little_string(buf, cursor)?;
        cursor += client_size;
        let login = Login {
            protocol_version,
            identity,
            client,
        };
        Ok(Packet {
            id: 1,
            kind: PacketTypes::Login(login),
            size: cursor - offset,
            buffer: buf.to_owned(),
        })
    }
}

#[derive(Debug)]
pub struct RequestNetworkSetting {
    pub client_protocol: i32,
}

impl RequestNetworkSetting {
    pub fn new(buf: &[u8], offset: u64) -> Result<Packet> {
        let (client_protocol, client_protocol_size) = read_i32(buf, offset)?;
        let rns = RequestNetworkSetting { client_protocol };
        Ok(Packet {
            id: 193,
            kind: PacketTypes::RequestNetworkSetting(rns),
            size: client_protocol_size,
            buffer: buf.to_owned(),
        })
    }
}

#[derive(Debug)]
pub struct PlayStatus {
    pub status: Status,
}

impl PlayStatus {
    pub fn compose(buffer: &mut Vec<u8>, packet: Self) -> Result<()> {
        write_i32(packet.status as i32, buffer)?;
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Status {
    LoginSuccess,
    FailedClient,
    FailedSpawn,
    PlayerSpawn,
    FailedInvalidTenant,
    FailedVanillaEdu,
    FailedEduVanilla,
    FailedServerFull,
    FailedEditorVanillaMismatch,
    FailedVanillaEditorMismatch,
}

#[derive(Debug)]
pub struct Disconnect {
    pub hide_disconnect_reason: bool,
    pub message: String,
}

impl Disconnect {
    pub fn compose(buffer: &mut Vec<u8>, packet: Self) -> Result<()> {
        write_bool(packet.hide_disconnect_reason, buffer).unwrap();
        write_string(packet.message, buffer).unwrap();
        Ok(())
    }
}

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
        write_lu16(packet.compression_threshold, buffer).unwrap();
        match packet.compression_algorithm {
            CompressionAlgorithmType::Deflate => write_lu16(0, buffer).unwrap(),
            CompressionAlgorithmType::Snappy => write_lu16(1, buffer).unwrap(),
        };
        write_bool(packet.client_throttle, buffer).unwrap();
        write_u8(packet.client_throttle_threshold, buffer).unwrap();
        write_lf32(packet.client_throttle_scalar, buffer).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompressionAlgorithmType {
    Deflate,
    Snappy,
}
