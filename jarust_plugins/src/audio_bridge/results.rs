use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioBridgePluginData {
    pub plugin: String,
    #[serde(rename = "data")]
    pub event: AudioBridgePluginEvent,
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
pub struct Participant {
    pub id: String,
    pub display: String,
    pub setup: bool,
    pub muted: bool,
    pub suspended: bool,
    pub talking: bool,
    pub spatial_position: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "audiobridge")]
pub enum AudioBridgePluginEvent {
    #[serde(rename = "created")]
    CreateRoom { room: u64, permanent: bool },
    #[serde(rename = "edited")]
    EditRoom { room: u64 },
    #[serde(rename = "destroyed")]
    DestroyRoom { room: u64, permanent: bool },
    #[serde(rename = "participants")]
    ListParticipants {
        room: u64,
        participants: Vec<Participant>,
    },
    #[serde(rename = "success")]
    #[serde(untagged)]
    List { list: Vec<Room> },
    #[serde(rename = "success")]
    #[serde(untagged)]
    Allowed { room: u64, allowed: Vec<String> },
    #[serde(rename = "success")]
    #[serde(untagged)]
    ExistsRoom { room: u64, exists: bool },
    #[serde(rename = "success")]
    #[serde(untagged)]
    Success {},
}
