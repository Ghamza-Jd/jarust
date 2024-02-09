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
    #[doc = "unique numeric ID, optional, chosen by plugin if missing"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<u64>,

    #[doc = "whether the room should be saved in the config file, default=false"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,

    #[doc = "pretty name of the room, optional"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[doc = "password required to edit/destroy the room, optional"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    #[doc = "password required to join the room, optional"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,

    #[doc = "whether the room should appear in a list request"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,

    #[doc = "array of string tokens users can use to join this room, optional"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Vec<String>>,

    #[doc = "sampling rate of the room, optional, 16000 by default"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<u64>,

    #[doc = "whether the mix should spatially place users, default=false"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_audio: Option<bool>,

    #[doc = "whether the ssrc-audio-level RTP extension must be negotiated for new joins, default=true"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiolevel_ext: Option<bool>,

    #[doc = "whether to emit event to other users or not"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiolevel_event: Option<bool>,

    #[doc = "number of packets with audio level (default=100, 2 seconds)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_active_packets: Option<u64>,

    #[doc = "average value of audio level (127=muted, 0='too loud', default=25)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_level_average: Option<u64>,

    #[doc = "percent of packets we expect participants may miss,"]
    #[doc = "to help with FEC (default=0, max=20; automatically used for forwarders too"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_expectedloss: Option<u64>,

    #[doc = "bitrate in bps to use for the all participants"]
    #[doc = "(default=0, which means libopus decides; automatically used for forwarders too)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_bitrate: Option<u64>,

    #[doc = "whether to record the room or not, default=false"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,

    #[doc = "/path/to/the/recording.wav, optional"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_file: Option<String>,

    #[doc = "/path/to/, optional; makes record_file a relative path, if provided"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_dir: Option<String>,

    #[doc = "whether all participants in the room should be individually recorded to mjr files, default=false"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mjrs: Option<bool>,

    #[doc = "/path/to/, optional"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mjrs_dir: Option<String>,

    #[doc = "whether participants should be allowed to join via plain RTP as well, default=false"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rtp_participants: Option<bool>,

    #[doc = "non-hierarchical array of string group names to use to gat participants,"]
    #[doc = "for external forwarding purposes only, optional"]
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
    #[doc = "room secret, mandatory if configured"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    #[doc = "new pretty name of the room, optional"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_description: Option<String>,

    #[doc = "new password required to edit/destroy the room, optional"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_secret: Option<String>,

    #[doc = "new PIN required to join the room, PIN will be removed if set to an empty string, optional"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_pin: Option<String>,

    #[doc = "whether the room should appear in a list request"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_is_private: Option<bool>,

    #[doc = "new path where new recording files should be saved"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_record_dir: Option<String>,

    #[doc = "new path where new MJR files should be saved"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_mjrs_dir: Option<String>,

    #[doc = "whether the room should be also removed from the config file, default=false"]
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

//
// Join Message
//

#[derive(Serialize, Default)]
pub struct AudioBridgeJoinMsg {
    request: String,
    pub room: u64,
    #[serde(flatten)]
    options: AudioBridgeJoinOptions,
}

#[derive(Serialize, Default)]
pub struct AudioBridgeJoinOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_loss: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_level_average: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_active_packets: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_offer: Option<bool>,
}

impl AudioBridgeJoinMsg {
    pub fn new(room: u64, options: AudioBridgeJoinOptions) -> Self {
        Self {
            request: "join".to_string(),
            room,
            options,
        }
    }
}

//
// Allowed Message
//

#[derive(Serialize)]
pub struct AudioBridgeAllowedMsg {
    request: String,
    pub room: u64,
    pub action: AudioBridgeAction,
    pub allowed: Vec<String>,
    #[serde(flatten)]
    pub options: AudioBridgeAllowedOptions,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioBridgeAction {
    Enable,
    Disable,
    Add,
    Remove,
}

#[derive(Serialize, Default)]
pub struct AudioBridgeAllowedOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl AudioBridgeAllowedMsg {
    pub fn new(
        room: u64,
        action: AudioBridgeAction,
        allowed: Vec<String>,
        options: AudioBridgeAllowedOptions,
    ) -> Self {
        Self {
            request: "allowed".to_string(),
            room,
            action,
            allowed,
            options,
        }
    }
}

//
// Kick Message
//

#[derive(Serialize)]
pub struct AudioBridgeKickMsg {
    request: String,
    pub room: u64,
    pub id: u64,
    #[serde(flatten)]
    pub options: AudioBridgeKickOptions,
}

#[derive(Serialize, Default)]
pub struct AudioBridgeKickOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl AudioBridgeKickMsg {
    pub fn new(room: u64, participant: u64, options: AudioBridgeKickOptions) -> Self {
        Self {
            request: "kick".to_string(),
            room,
            id: participant,
            options,
        }
    }
}

//
// Kick All Message
//

#[derive(Serialize)]
pub struct AudioBridgeKickAllMsg {
    request: String,
    pub room: u64,
    #[serde(flatten)]
    pub options: AudioBridgeKickAllOptions,
}

#[derive(Serialize, Default)]
pub struct AudioBridgeKickAllOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl AudioBridgeKickAllMsg {
    pub fn new(room: u64, options: AudioBridgeKickAllOptions) -> Self {
        Self {
            request: "kick_all".to_string(),
            room,
            options,
        }
    }
}

//
// Suspend Message
//

#[derive(Serialize)]
pub struct AudioBridgeSuspendMsg {
    request: String,
    pub room: u64,
    pub id: u64,
    #[serde(flatten)]
    pub options: AudioBridgeSuspendOptions,
}

#[derive(Serialize, Default)]
pub struct AudioBridgeSuspendOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_record: Option<bool>,
}

impl AudioBridgeSuspendMsg {
    pub fn new(room: u64, participant: u64, options: AudioBridgeSuspendOptions) -> Self {
        Self {
            request: "suspend".to_string(),
            room,
            id: participant,
            options,
        }
    }
}

//
// Resume Message
//

#[derive(Serialize)]
pub struct AudioBridgeResumeMsg {
    request: String,
    pub room: u64,
    pub id: u64,
    #[serde(flatten)]
    pub options: AudioBridgeResumeOptions,
}

#[derive(Serialize, Default)]
pub struct AudioBridgeResumeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl AudioBridgeResumeMsg {
    pub fn new(room: u64, participant: u64, options: AudioBridgeResumeOptions) -> Self {
        Self {
            request: "resume".to_string(),
            room,
            id: participant,
            options,
        }
    }
}

//
// List Participants Message
//

#[derive(Serialize)]
pub struct AudioBridgeListParticipantsMsg {
    request: String,
    pub room: u64,
}

impl AudioBridgeListParticipantsMsg {
    pub fn new(room: u64) -> Self {
        Self {
            request: "listparticipants".to_string(),
            room,
        }
    }
}
