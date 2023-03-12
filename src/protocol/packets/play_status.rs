use anyhow::Result;
use byteorder::{BigEndian, WriteBytesExt};

#[derive(Debug)]
pub struct PlayStatus {
    pub status: Status,
}

impl PlayStatus {
    pub fn compose(buffer: &mut Vec<u8>, packet: Self) -> Result<()> {
        buffer.write_i32::<BigEndian>(packet.status as i32)?;
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
