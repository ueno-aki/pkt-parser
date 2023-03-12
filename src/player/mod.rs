use rust_raknet::RaknetSocket;
use std::net::SocketAddr;

use crate::protocol::{
    compresstion::{compose_packet, decompress, get_packets, parse_packet},
    login::login_verify::{verify_auth, verify_skin},
    packets::{
        login::Login,
        network_settings::{CompressionAlgorithmType, NetworkSettings},
        play_status::{PlayStatus, Status},
        Packet, PacketTypes,
    },
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
        while let Ok(buf) = self.socket.recv().await {
            if buf[0] == 0xfe {
                self.handle(buf[1..].to_vec()).await;
            }
        }
        println!("disconnected");
    }
    async fn handle(&self, buffer: Vec<u8>) {
        let flate: Vec<u8> = match decompress(&buffer) {
            Ok(buffer) => buffer,
            Err(_) => buffer,
        };

        let pkts: Vec<Vec<u8>> = get_packets(&flate).unwrap();
        for pkt in pkts {
            let parsed_pkt: Packet = parse_packet(&pkt, 0).unwrap();
            println!("client={},packet={}", self.address.to_string(), parsed_pkt);

            match parsed_pkt.kind {
                PacketTypes::RequestNetworkSetting(pkt) => match pkt.client_protocol {
                    n if n > 568 => self.disconnect_with_status(Status::FailedSpawn).await,
                    _ => self.send_network_settings().await,
                },
                PacketTypes::Login(pkt) => {
                    self.decode_jwt(pkt);
                    self.socket.close().await.unwrap();
                }
                _ => todo!(),
            };
        }
    }

    pub async fn send_packet(&self, packet: Packet) {
        println!("client={},packet={}", self.address.to_string(), packet);
        self.socket
            .send(
                &[vec![0xfe], compose_packet(packet).unwrap()].concat(),
                rust_raknet::Reliability::ReliableOrdered,
            )
            .await
            .unwrap();
    }

    async fn send_network_settings(&self) {
        let network = PacketTypes::NetworkSettings(NetworkSettings {
            compression_threshold: 512,
            compression_algorithm: CompressionAlgorithmType::Deflate,
            client_throttle: false,
            client_throttle_threshold: 0,
            client_throttle_scalar: 0.0,
        });
        self.send_packet(network.into()).await;
    }

    async fn disconnect_with_status(&self, status: Status) {
        let failed_spawn = PacketTypes::PlayStatus(PlayStatus { status });
        self.send_packet(failed_spawn.into()).await;
        self.socket.flush().await.unwrap();
        self.socket.close().await.unwrap();
    }

    fn decode_jwt(&self, login: Login) {
        let v = verify_auth(&login.identity).unwrap();
        let skin_data = verify_skin(&v.key, &login.client).unwrap();
        println!("{:?},{:?}", v, skin_data);
    }
}
