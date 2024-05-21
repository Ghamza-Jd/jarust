use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RoomCreatedRsp {
    pub room: u64,
    pub permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct RoomEditedRsp {
    pub room: u64,
    pub permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct RoomDestroyedRsp {
    pub room: u64,
    pub permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListRoomsRsp {
    pub list: Vec<Room>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    pub room: u64,
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
    pub room: u64,
    pub allowed: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExistsRoomRsp {
    pub room: u64,
    pub exists: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListParticipantsRsp {
    pub room: u64,
    pub participants: Vec<Participant>,
}

#[derive(Debug, Deserialize)]
pub struct Participant {
    pub id: u64,
    pub display: Option<String>,
    pub setup: bool,
    pub muted: bool,
    pub suspended: Option<bool>,
    pub talking: Option<bool>,
    pub spatial_position: Option<u64>,
}
