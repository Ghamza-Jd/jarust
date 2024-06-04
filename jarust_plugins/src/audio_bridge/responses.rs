use super::common::Identifier;
use super::common::Participant;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RoomCreatedRsp {
    pub room: Identifier,
    pub permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct RoomEditedRsp {
    pub room: Identifier,
    pub permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct RoomDestroyedRsp {
    pub room: Identifier,
    pub permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListRoomsRsp {
    pub list: Vec<Room>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    pub room: Identifier,
    pub description: String,
    pub pin_required: bool,
    pub sampling_rate: u64,
    pub spatial_audio: Option<bool>,
    pub record: bool,
    pub num_participants: u64,
    pub muted: bool,
}

#[derive(Debug, Deserialize)]
pub struct AllowedRsp {
    pub room: Identifier,
    pub allowed: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExistsRoomRsp {
    pub room: Identifier,
    pub exists: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListParticipantsRsp {
    pub room: Identifier,
    pub participants: Vec<Participant>,
}
