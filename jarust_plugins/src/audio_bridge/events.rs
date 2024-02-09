use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioBridgePluginData {
    pub plugin: String,
    #[serde(rename = "data")]
    pub event: AudioBridgePluginEvent,
}

#[derive(Debug, Deserialize)]
pub struct Participant {
    pub id: u64,
    pub display: Option<String>,
    pub setup: bool,
    pub muted: bool,
    pub suspended: bool,
    pub talking: bool,
    pub spatial_position: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "audiobridge")]
pub enum AudioBridgePluginEvent {
    #[serde(rename = "joined")]
    JoinRoom {
        id: u64,
        room: u64,
        participants: Vec<Participant>,
    },
}
