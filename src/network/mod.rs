pub mod login;
pub mod packet;

use std::fmt::Display;

use packet::*;
#[derive(Debug)]
pub struct Packet {
    pub id: u64,
    pub kind: PacketTypes,
    pub size: u64,
    pub buffer: Vec<u8>,
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            1 => write!(f, "[C->S] LoginPacket"),
            2 => write!(f, "[S -> C] PlayStatusPacket"),
            143 => write!(f, "[S->C] NetworkSettingsPacket"),
            193 => write!(f, "[C->S] RequestNetworkSettingPacket"),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum PacketTypes {
    Login(Login),
    PlayStatus(PlayStatus),
    Disconnect(Disconnect),
    RequestNetworkSetting(RequestNetworkSetting),
    NetworkSettings(NetworkSettings),
}
