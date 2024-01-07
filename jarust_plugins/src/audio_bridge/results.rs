use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AudioBridgePluginData {
    pub plugin: String,
    #[serde(rename = "data")]
    pub event: AudioBridgePluginEvent,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    room: u64,
    description: String,
    pin_required: bool,
    sampling_rate: u64,
    spatial_audio: bool,
    record: bool,
    num_participants: u64,
    muted: bool,
}

#[derive(Debug, Deserialize)]
pub enum AudioBridgePluginEvent {
    #[serde(untagged)]
    List {
        audiobridge: String,
        list: Vec<Room>,
    },
}
