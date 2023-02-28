use std::{
    io::{prelude::*, Result},
    net::SocketAddr,
};

use rust_raknet::RaknetSocket;

use crate::{
    network::{
        packet::{CompressionAlgorithmType, Login, NetworkSettings, RequestNetworkSetting},
        Packet, PacketTypes,
    },
    protodef::native_types::{reader::read_varint, writer::write_var_int},
};

pub struct Player {
    address: SocketAddr,
    socket: RaknetSocket,
}
impl Player {
    pub fn new(socket: RaknetSocket) -> Player {
        Player {
            address: socket.peer_addr().unwrap(),
            socket,
        }
    }

    pub async fn listen(&self) {
        loop {
            if let Ok(buf) = self.socket.recv().await {
                if buf[0] == 0xfe {
                    self.handle(buf[1..].to_vec()).await;
                }
            } else {
                println!("disconnected");
                break;
            }
        }
    }
    async fn handle(&self, buffer: Vec<u8>) {
        let flate = match decompress(&buffer) {
            Ok(buffer) => buffer,
            Err(_) => buffer,
        };

        let pkts = get_packets(&flate).unwrap();
        for pkt in pkts {
            let parsed_pkt = parse_packet(&pkt, 0).unwrap();
            println!("client={},packet={}", self.address.to_string(), parsed_pkt);

            match parsed_pkt.kind {
                PacketTypes::RequestNetworkSetting(_) => {
                    self.send_network_settings().await;
                }
                _ => {}
            };
        }
    }

    async fn send_network_settings(&self) {
        let network = Packet {
            id: 143,
            kind: PacketTypes::NetworkSettings(NetworkSettings {
                compression_threshold: 512,
                compression_algorithm: CompressionAlgorithmType::Deflate,
                client_throttle: false,
                client_throttle_threshold: 0,
                client_throttle_scalar: 0.0,
            }),
            size: 0,
            buffer: vec![0],
        };
        println!("client={},packet={}", self.address.to_string(), network);

        self.socket
            .send(
                &[vec![0xfe], compose_packet(network).unwrap()].concat(),
                rust_raknet::Reliability::ReliableOrdered,
            )
            .await
            .unwrap();
    }
}

pub fn decompress(buf: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = flate2::bufread::DeflateDecoder::new(&buf[..]);
    let mut s: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut s)?;
    Ok(s)
}

pub fn get_packets(buf: &[u8]) -> Result<Vec<Vec<u8>>> {
    let mut packets = Vec::new();
    let mut offset: u64 = 0;

    while offset < buf.len().try_into().unwrap() {
        let (value, size) = read_varint(buf, offset)?;
        let mut dec: Vec<u8> = vec![0; value as usize];
        offset += size;
        let edge = offset + value;
        dec.copy_from_slice(&buf[(offset as usize)..(edge as usize)]);
        offset += value;
        packets.push(dec);
    }
    Ok(packets)
}

pub fn parse_packet(buf: &[u8], offset: u64) -> Result<Packet> {
    let (name_value, name_size) = read_varint(buf, offset)?;
    let mut x = match name_value {
        1 => Login::new(buf, offset + name_size)?,
        193 => RequestNetworkSetting::new(buf, offset + name_size)?,
        _ => todo!(),
    };
    x.size += name_size;
    Ok(x)
}

pub fn compose_packet(packet: Packet) -> Result<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();
    write_var_int(packet.id, &mut buffer).unwrap();
    match packet.kind {
        PacketTypes::NetworkSettings(pkt) => NetworkSettings::compose(&mut buffer, pkt).unwrap(),
        _ => todo!(),
    };
    let mut result: Vec<u8> = Vec::new();
    write_var_int(buffer.len() as u64, &mut result).unwrap();
    Ok([result, buffer].concat())
}

// pub fn compress(buf: &[u8]) -> Result<Vec<u8>> {
//     let mut encoder = flate2::bufread::DeflateEncoder::new(&buf[..],Compression::new(7));
//     let mut s:Vec<u8> = Vec::new();
//     encoder.read_to_end(&mut s)?;
//     Ok(s)
// }
