use crate::JanusId;
use serde::Serialize;

impl_tryfrom_serde_value!(
    AudioBridgeCreateRoomOptions AudioBridgeEditRoomOptions AudioBridgeDestroyRoomMsg
    AudioBridgeJoinRoomOptions AudioBridgeAllowedOptions AudioBridgeAllowAction
    AudioBridgeConfigureOptions AudioBridgeMuteOptions AudioBridgeMuteRoomOptions
    AudioBridgeKickOptions AudioBridgeKickAllOptions AudioBridgeChangeRoomOptions
);

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct AudioBridgeCreateRoomOptions {
    /// unique numeric ID, chosen by plugin if missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<JanusId>,

    /// whether the room should be saved in the config file, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,

    /// pretty name of the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// password required to edit/destroy the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// password required to join the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,

    /// whether the room should appear in a list request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,

    /// array of string tokens users can use to join this room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Vec<String>>,

    /// sampling rate of the room, 16000 by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<u64>,

    /// whether the mix should spatially place users, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_audio: Option<bool>,

    /// whether the ssrc-audio-level RTP extension must be negotiated for new joins, default=true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiolevel_ext: Option<bool>,

    /// whether to emit event to other users or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiolevel_event: Option<bool>,

    /// number of packets with audio level (default=100, 2 seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_active_packets: Option<u64>,

    /// average value of audio level (127=muted, 0='too loud', default=25)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_level_average: Option<u64>,

    /// percent of packets we expect participants may miss,
    /// to help with FEC (default=0, max=20; automatically used for forwarders too
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_expectedloss: Option<u64>,

    /// bitrate in bps to use for the all participants
    /// (default=0, which means libopus decides; automatically used for forwarders too)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_bitrate: Option<u64>,

    /// whether to record the room or not, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,

    /// /path/to/the/recording.wav
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_file: Option<String>,

    /// /path/to/; makes record_file a relative path, if provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_dir: Option<String>,

    /// whether all participants in the room should be individually recorded to mjr files, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mjrs: Option<bool>,

    /// /path/to/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mjrs_dir: Option<String>,

    /// whether participants should be allowed to join via plain RTP as well, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rtp_participants: Option<bool>,

    /// non-hierarchical array of string group names to use to gat participants,
    /// for external forwarding purposes only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct AudioBridgeEditRoomOptions {
    pub room: JanusId,

    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// new pretty name of the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_description: Option<String>,

    /// new password required to edit/destroy the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_secret: Option<String>,

    /// new PIN required to join the room, PIN will be removed if set to an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_pin: Option<String>,

    /// whether the room should appear in a list request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_is_private: Option<bool>,

    /// new path where new recording files should be saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_record_dir: Option<String>,

    /// new path where new MJR files should be saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_mjrs_dir: Option<String>,

    /// whether the room should be also removed from the config file, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct AudioBridgeDestroyRoomMsg {
    pub room: JanusId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct AudioBridgeJoinRoomOptions {
    pub room: JanusId,

    /// Unique ID to assign to the participant, assigned by the plugin if missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<JanusId>,

    /// Group to assign to this participant (for forwarding purposes only; optional, mandatory if enabled in the room)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// Password required to join the room, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,

    /// Display name to have in the room.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// Invitation token, in case the room has an ACL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// Whether to start unmuted or muted, false by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,

    /// Whether to start suspended or not, false by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,

    /// Whether room events should be paused, if the user is joining as suspended, false by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_events: Option<bool>,

    /// Codec to use, among opus (default), pcma (A-Law) or pcmu (mu-Law)"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,

    /// Bitrate to use for the Opus stream in bps, default=0 (libopus decides)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,

    /// 0-10, Opus-related complexity to use, the higher the value, the better the quality (but more CPU); default is 4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u64>,

    /// 0-20, a percentage of the expected loss (capped at 20%), only needed in case FEC is used; optional,
    /// default is 0 (FEC disabled even when negotiated) or the room default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_loss: Option<u64>,

    // Percent value, <100 reduces volume, >100 increases volume; optional, default is 100 (no volume change)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<u64>,

    /// In case spatial audio is enabled for the room, panning of this participant (0=left, 50=center, 100=right)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_position: Option<String>,

    /// Room management password, if provided the user is an admin and can't be globally muted with `mute_room`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// If provided, overrides the room `audio_level_average` for this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_level_average: Option<u64>,

    /// If provided, overrides the room audio_active_packets for this user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_active_packets: Option<u64>,

    /// Whether to record this user's contribution to a .mjr file (mixer not involved)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,

    /// Basename of the file to record to, -audio.mjr will be added by the plugin; will be relative to mjrs_dir,
    /// if configured in the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// A user can ask the plugin to generate an SDP offer first, to which they'd provide an SDP answer to.
    /// This slightly changes the way the negotiation works within the context of the AudioBridge API, as some messages
    /// may have to be used in a different way.
    ///
    /// This means that the user will receive a JSEP SDP offer as part of the related event: at this
    /// point, the user needs to prepare to send a JSEP SDP answer and send it back to the plugin to complete the
    /// negotiation. The user must use the `configure` request to provide this SDP answer: no need to provide
    /// additional attributes in the request, unless it's needed for application related purposes (e.g., to start muted).
    ///
    /// Notice that this does have an impact on renegotiations, e.g., for ICE restarts or changes in the media direction.
    /// As a policy, plugins in Janus tend to enforce the same negotiation pattern used to setup the PeerConnection
    /// initially for renegotiations too, as it reduces the risk of issues like glare: this means that users will NOT be
    /// able to send an SDP offer to the AudioBridge plugin to update an existing PeerConnection, if that PeerConnection
    /// had previously been originated by a plugin offer instead. The plugin will treat this as an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_offer: Option<bool>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct AudioBridgeAllowedOptions {
    pub room: JanusId,

    pub action: AudioBridgeAllowAction,

    /// Array of strings (tokens users might pass in "join", only for add|remove)
    pub allowed: Vec<String>,

    /// Room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioBridgeAllowAction {
    Enable,
    Disable,
    Add,
    Remove,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Serialize, Default)]
pub struct AudioBridgeConfigureOptions {
    /// whether to unmute or mute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,

    /// new display name to have in the room (see "join" for more info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// new bitrate to use for the Opus stream
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,

    /// new Opus-related complexity to use (see "join" for more info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u64>,

    /// new value for the expected loss (see "join" for more info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_loss: Option<u64>,

    /// new volume percent value (see "join" for more info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<u64>,

    /// in case spatial audio is enabled for the room, new panning of this participant (0=left, 50=center, 100=right)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_position: Option<u64>,

    /// whether denoising via RNNoise should be performed for this participant (default=room value)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise: Option<bool>,

    /// whether to record this user's contribution to a .mjr file (mixer not involved)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,

    /// basename of the file to record to, -audio.mjr will be added by the plugin; will be relative to mjrs_dir
    /// if configured in the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// new group to assign to this participant, if enabled in the room (for forwarding purposes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Serialize)]
pub struct AudioBridgeMuteOptions {
    /// unique numeric ID
    pub id: JanusId,

    /// unique numeric ID
    pub room: JanusId,

    /// Room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Serialize)]
pub struct AudioBridgeMuteRoomOptions {
    /// unique numeric ID
    pub room: JanusId,

    /// Room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Serialize)]
pub struct AudioBridgeKickOptions {
    /// unique numeric ID
    pub id: JanusId,

    /// unique numeric ID
    pub room: JanusId,

    /// Room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Serialize)]
pub struct AudioBridgeKickAllOptions {
    /// unique numeric ID
    pub room: JanusId,

    /// Room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Serialize)]
pub struct AudioBridgeChangeRoomOptions {
    pub room: JanusId,

    /// numeric ID of the room to move to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<JanusId>,

    /// whether to unmute or mute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,

    /// new display name to have in the room (see "join" for more info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// new bitrate to use for the Opus stream
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,

    /// new Opus-related complexity to use (see "join" for more info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u64>,

    /// new value for the expected loss (see "join" for more info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_loss: Option<u64>,

    /// new volume percent value (see "join" for more info)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<u64>,

    /// in case spatial audio is enabled for the room, new panning of this participant (0=left, 50=center, 100=right)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_position: Option<u64>,

    /// whether denoising via RNNoise should be performed for this participant (default=room value)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise: Option<bool>,
}
