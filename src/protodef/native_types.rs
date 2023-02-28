pub mod reader {
    use std::io::Result;

    use byteorder::{BigEndian, ByteOrder, LittleEndian};

    pub fn read_i32(buf: &[u8], offset: u64) -> Result<(i32, u64)> {
        let values: i32 = BigEndian::read_i32(&buf[(offset as usize)..]);
        let result: (i32, u64) = (values, 4);
        Ok(result)
    }

    pub fn read_li32(buf: &[u8], offset: u64) -> Result<(i32, u64)> {
        let values: i32 = LittleEndian::read_i32(&buf[(offset as usize)..]);
        let result: (i32, u64) = (values, 4);
        Ok(result)
    }

    pub fn read_varint(buf: &[u8], offset: u64) -> Result<(u64, u64)> {
        let mut result: u64 = 0;
        let mut shift: u8 = 0;
        let mut cursor = offset;
        let temp: (u64, u64);
        loop {
            if ((cursor + 1) as usize) > buf.len() {
                panic!()
            } else {
                let b = buf[cursor as usize];
                result |= (b as u64 & 0x7f) << shift;
                cursor += 1;
                if (b as u64 & 0x80) == 0 {
                    temp = (result, cursor - offset);
                    break;
                }
                shift += 7;
            }
        }
        Ok(temp)
    }
}

pub mod writer {
    use std::io::Result;

    use byteorder::{LittleEndian, WriteBytesExt};

    pub fn write_u8(value: u8, buf: &mut Vec<u8>) -> Result<()> {
        buf.write_u8(value).unwrap();
        Ok(())
    }

    pub fn write_lu16(value: u16, buf: &mut Vec<u8>) -> Result<()> {
        buf.write_u16::<LittleEndian>(value).unwrap();
        Ok(())
    }

    pub fn write_lf32(value: f32, buf: &mut Vec<u8>) -> Result<()> {
        buf.write_f32::<LittleEndian>(value).unwrap();
        Ok(())
    }

    pub fn write_var_int(value: u64, buf: &mut Vec<u8>) -> Result<u64> {
        let mut cursor: u64 = 0;
        let mut v: u64 = value.clone();
        while (v & !0x7f) != 0 {
            buf.write_u8(((v & 0xff) | 0x80) as u8).unwrap();
            cursor += 1;
            v >>= 7;
        }
        buf.write_u8(v as u8).unwrap();
        Ok(cursor + 1)
    }

    pub fn write_bool(value: bool, buf: &mut Vec<u8>) -> Result<()> {
        buf.write_i8(value as i8).unwrap();
        Ok(())
    }
}
