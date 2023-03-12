#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use anyhow::Result;
    use byteorder::{LittleEndian, WriteBytesExt};
    use serde::{Deserialize, Serialize};

    use crate::{
        protocol::compresstion::{decompress, get_packets, parse_packet},
        protodef::writer::Write,
    };

    #[derive(Serialize, Deserialize)]
    struct Buffer {
        data: Vec<u8>,
    }

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
    fn write_native_types() -> Result<()> {
        let buf = [0x8e, 0xdc, 0x08];
        let mut buffer: Vec<u8> = Vec::new();
        buffer.write_var_int(142862)?;
        buffer.write_u16::<LittleEndian>(44829)?;
        buffer.write_bool(true)?;
        buffer.write_string("abcdeg".to_owned())?;
        println!("{:?},{:?}", buf, buffer);
        Ok(())
    }
}
