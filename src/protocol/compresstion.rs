use super::packets::{
    login::Login, network_settings::NetworkSettings, play_status::PlayStatus,
    request_network_settings::RequestNetworkSetting, Packet, PacketTypes,
};
use crate::protodef::{reader::Read as _, writer::Write};
use anyhow::Result;
use std::io::Read;

pub fn decompress(buf: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = flate2::bufread::DeflateDecoder::new(&buf[..]);
    let mut s: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut s)?;
    Ok(s)
}

pub fn get_packets(buf: &[u8]) -> Result<Vec<Vec<u8>>> {
    let mut packets: Vec<Vec<u8>> = Vec::new();
    let mut offset: u64 = 0;

    while offset < buf.len().try_into().unwrap() {
        let (value, size) = buf.read_varint(offset)?;
        let mut dec: Vec<u8> = vec![0; value as usize];
        offset += size;
        let edge = (offset + value) as usize;
        dec.copy_from_slice(&buf[(offset as usize)..edge]);
        offset += value;
        packets.push(dec);
    }
    Ok(packets)
}

pub fn parse_packet(buf: &[u8], offset: u64) -> Result<Packet> {
    let (name_value, name_size) = buf.read_varint(offset)?;
    let x: PacketTypes = match name_value {
        1 => PacketTypes::Login(Login::from_vec_u8(buf, offset + name_size)?),
        193 => PacketTypes::RequestNetworkSetting(RequestNetworkSetting::from_vec_u8(
            buf,
            offset + name_size,
        )?),
        _ => todo!(),
    };
    Ok(x.into())
}

pub fn compose_packet(packet: Packet) -> Result<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write_var_int(packet.id)?;
    match packet.kind {
        PacketTypes::NetworkSettings(pkt) => NetworkSettings::compose(&mut buffer, pkt)?,
        PacketTypes::PlayStatus(pkt) => PlayStatus::compose(&mut buffer, pkt)?,
        _ => todo!(),
    };
    let mut result: Vec<u8> = Vec::new();
    result.write_var_int(buffer.len() as u64)?;
    Ok([result, buffer].concat())
}

// pub fn compress(buf: &[u8]) -> Result<Vec<u8>> {
//     let mut encoder = flate2::bufread::DeflateEncoder::new(&buf[..],Compression::new(7));
//     let mut s:Vec<u8> = Vec::new();
//     encoder.read_to_end(&mut s)?;
//     Ok(s)
// }
