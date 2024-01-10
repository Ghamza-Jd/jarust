use serde::Serialize;

#[derive(Serialize)]
pub struct AudioBridgeStartMsg {
    pub audio: bool,
    pub video: bool,
}

#[derive(Serialize)]
pub struct AudioBridgeCreateMsg {
    request: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiolevel_ext: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiolevel_event: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_active_packets: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_level_average: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_expectedloss: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_bitrate: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mjrs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mjrs_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rtp_participants: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
}

impl AudioBridgeCreateMsg {
    pub fn new(
        room: Option<u64>,
        permanent: Option<bool>,
        description: Option<String>,
        secret: Option<String>,
        pin: Option<String>,
        is_private: Option<bool>,
        allowed: Option<Vec<String>>,
        sampling_rate: Option<u64>,
        spatial_audio: Option<bool>,
        audiolevel_ext: Option<bool>,
        audiolevel_event: Option<bool>,
        audio_active_packets: Option<u64>,
        audio_level_average: Option<u64>,
        default_expectedloss: Option<u64>,
        default_bitrate: Option<u64>,
        record: Option<bool>,
        record_file: Option<String>,
        record_dir: Option<String>,
        mjrs: Option<bool>,
        mjrs_dir: Option<String>,
        allow_rtp_participants: Option<bool>,
        groups: Option<Vec<String>>,
    ) -> Self {
        Self {
            request: "create".to_string(),
            room,
            permanent,
            description,
            secret,
            pin,
            is_private,
            allowed,
            sampling_rate,
            spatial_audio,
            audiolevel_ext,
            audiolevel_event,
            audio_active_packets,
            audio_level_average,
            default_expectedloss,
            default_bitrate,
            record,
            record_file,
            record_dir,
            mjrs,
            mjrs_dir,
            allow_rtp_participants,
            groups,
        }
    }
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
