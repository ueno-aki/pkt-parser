mod network;
mod player;
mod protodef;
use player::*;

use rust_raknet::RaknetListener;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let mut listener = RaknetListener::bind(&"0.0.0.0:19132".parse().unwrap())
        .await
        .unwrap();
    listener
        .set_motd("rust-raknet", 20, "567", "1.19.62", "Survival", 19132)
        .await;
    listener.listen().await;
    while let Ok(socket) = listener.accept().await {
        tokio::spawn(async move {
            let player = Player::new(socket);
            player.listen().await;
        });
    }
}
#[cfg(test)]
mod tests {
    use crate::protodef::{
        mcbe::writer::write_string,
        native_types::writer::{write_bool, write_lu16, write_var_int},
    };

    use super::*;
    use std::{fs::File, io::Read};
    #[test]
    #[ignore]
    fn buffer_read() {
        let mut file = File::open("buf2.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let serialize: Buffer = serde_json::from_str(&contents).unwrap();
        let buffer = serialize.data.clone();
        // let buffer: Vec<u8> = vec![0x06, 0xc1, 0x01, 0x00, 0x00, 0x02, 0x37];
        let flate = match decompress(&buffer) {
            Ok(buf) => buf,
            Err(_) => buffer,
        };

        let pkts = get_packets(&flate).unwrap();
        for pkt in pkts {
            let parsed_pkt = parse_packet(&pkt, 0).unwrap();
            println!("{:?}", parsed_pkt);
        }
    }

    #[test]
    fn write_native_types() {
        let buf = [0x8e, 0xdc, 0x08];
        let mut buffer: Vec<u8> = Vec::new();
        write_var_int(142862, &mut buffer).unwrap();
        write_lu16(44829, &mut buffer).unwrap();
        write_bool(false, &mut buffer).unwrap();
        write_string("abcdeg".to_owned(), &mut buffer).unwrap();
        println!("{:?},{:?}", buf, buffer);
    }
}

#[derive(Serialize, Deserialize)]
struct Buffer {
    data: Vec<u8>,
}
