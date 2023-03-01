pub mod reader {
    use crate::protodef::native_types::reader::read_li32;
    use std::io::Result;

    pub fn write_little_string(buf: &[u8], offset: u64) -> Result<(String, u64)> {
        let mut cursor: u64 = offset;
        let (value, size) = read_li32(buf, offset).unwrap();
        cursor += size;
        if cursor + (value as u64) > buf.len().try_into().unwrap() {
            eprintln!(
                "Error:Missing characters in string, found size is {}, expected size was {}",
                buf.len(),
                cursor + (value as u64)
            );
        }
        let edge = cursor + (value as u64);
        let str = String::from_utf8(buf[(cursor as usize)..(edge as usize)].to_vec()).unwrap();
        let size = size + (value as u64);
        Ok((str, size))
    }
}

pub mod writer {
    use std::io::Result;

    use crate::protodef::native_types::writer::write_var_int;

    pub fn write_string(value: String, buf: &mut Vec<u8>) -> Result<u64> {
        let mut cursor = 0;
        let len = value.as_bytes().len() as u64;
        cursor += write_var_int(len, buf).unwrap();
        buf.append(&mut value.as_bytes().to_vec());
        Ok(cursor + len)
    }
}
