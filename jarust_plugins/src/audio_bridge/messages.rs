use serde::Serialize;

#[derive(Serialize)]
pub struct AudioBridgeStartMsg {
    pub audio: bool,
    pub video: bool,
}

#[derive(Serialize)]
pub struct AudioBridgeCreateMsg {
    pub request: String,
    pub room: bool,
    pub permanent: bool,
    pub description: bool,
    pub secret: bool,
    pub pin: bool,
    pub is_private: bool,
    pub allowed: bool,
    pub sampling_rate: bool,
    pub spatial_audio: bool,
    pub audiolevel_ext: bool,
    pub audiolevel_event: bool,
    pub audio_active_packets: bool,
    pub audio_level_average: bool,
    pub default_expectedloss: bool,
    pub default_bitrate: bool,
    pub record: bool,
    pub record_file: bool,
    pub record_dir: bool,
    pub mjrs: bool,
    pub mjrs_dir: bool,
    pub allow_rtp_participants: bool,
    pub groups: bool,
}

#[derive(Serialize)]
pub struct AudioBridgeListMsg {
    pub request: String,
}
impl Default for AudioBridgeListMsg {
    fn default() -> Self {
        Self {
            request: "list".to_string(),
        }
    }
}
