use serde::Serialize;

//
// Create Message
//

#[derive(Serialize, Default)]
pub struct AudioBridgeCreateMsg {
    request: String,
    #[serde(flatten)]
    options: AudioBridgeCreateOptions,
}

#[derive(Serialize, Default)]
pub struct AudioBridgeCreateOptions {
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
    pub fn new(options: AudioBridgeCreateOptions) -> Self {
        Self {
            request: "create".to_string(),
            options,
        }
    }
}

//
// List Message
//

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

//
// Edit Message
//

#[derive(Serialize, Default)]
pub struct AudioBridgeEditMsg {
    request: String,
    pub room: u64,
    #[serde(flatten)]
    options: AudioBridgeEditOptions,
}

#[derive(Serialize, Default)]
pub struct AudioBridgeEditOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_pin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_record_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_mjrs_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

impl AudioBridgeEditMsg {
    pub fn new(room: u64, options: AudioBridgeEditOptions) -> Self {
        Self {
            request: "edit".to_string(),
            room,
            options,
        }
    }
}

//
// Destroy Message
//

#[derive(Serialize, Default)]
pub struct AudioBridgeDestroyMsg {
    request: String,
    pub room: u64,
    #[serde(flatten)]
    options: AudioBridgeDestroyOptions,
}

#[derive(Serialize, Default)]
pub struct AudioBridgeDestroyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

impl AudioBridgeDestroyMsg {
    pub fn new(room: u64, options: AudioBridgeDestroyOptions) -> Self {
        Self {
            request: "destroy".to_string(),
            room,
            options,
        }
    }
}

//
// Exists Message
//

#[derive(Serialize, Default)]
pub struct AudioBridgeExistsMsg {
    request: String,
    pub room: u64,
}

impl AudioBridgeExistsMsg {
    pub fn new(room: u64) -> Self {
        Self {
            request: "exists".to_string(),
            room,
        }
    }
}
