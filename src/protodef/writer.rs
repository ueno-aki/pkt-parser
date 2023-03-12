use anyhow::Result;
use byteorder::WriteBytesExt;

pub trait Write {
    fn write_var_int(&mut self, value: u64) -> Result<u64>;
    fn write_bool(&mut self, value: bool) -> Result<()>;
    fn write_string(&mut self, value: String) -> Result<u64>;
}
impl Write for Vec<u8> {
    fn write_var_int(&mut self, value: u64) -> Result<u64> {
        let mut cursor: u64 = 0;
        let mut v: u64 = value.clone();
        while (v & !0x7f) != 0 {
            self.write_u8(((v & 0xff) | 0x80) as u8)?;
            cursor += 1;
            v >>= 7;
        }
        self.write_u8(v as u8)?;
        Ok(cursor + 1)
    }
    fn write_bool(&mut self, value: bool) -> Result<()> {
        self.write_i8(value.into())?;
        Ok(())
    }
    fn write_string(&mut self, value: String) -> Result<u64> {
        let mut cursor = 0;
        let len = value.as_bytes().len() as u64;
        cursor += self.write_var_int(len)?;
        self.append(&mut value.as_bytes().to_vec());
        Ok(cursor + len)
    }
}
