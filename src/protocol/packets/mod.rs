pub mod login;
pub mod network_settings;
pub mod play_status;
pub mod request_network_settings;

use self::{
    login::Login, network_settings::NetworkSettings, play_status::PlayStatus,
    request_network_settings::RequestNetworkSetting,
};

#[derive(Debug)]
pub enum PacketTypes {
    Login(Login),
    PlayStatus(PlayStatus),
    RequestNetworkSetting(RequestNetworkSetting),
    NetworkSettings(NetworkSettings),
}

#[derive(Debug)]
pub struct Packet {
    pub id: u64,
    pub kind: PacketTypes,
}

impl std::fmt::Display for Packet {
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
impl From<PacketTypes> for Packet {
    fn from(value: PacketTypes) -> Self {
        let id: u64 = match value {
            PacketTypes::Login(_) => 1,
            PacketTypes::PlayStatus(_) => 2,
            PacketTypes::NetworkSettings(_) => 143,
            PacketTypes::RequestNetworkSetting(_) => 193,
        };
        Packet { id, kind: value }
    }
}
