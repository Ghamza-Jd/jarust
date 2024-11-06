use crate::JanusId;
use serde::Serialize;

make_dto!(
    AudioBridgeCreateParams,
    optional {
        /// Room identifier, chosen by plugin if missing
        room: JanusId,
        /// Whether the room should be saved in the config file, default=false
        permanent: bool,
        /// pretty name of the room
        description: String,
        /// password required to edit/destroy the room
        secret: String,
        /// password required to join the room
        pin: String,
        /// whether the room should appear in a list request
        is_private: bool,
        /// array of string tokens users can use to join this room
        allowed: Vec<String>,
        /// sampling rate of the room, 16000 by default
        sampling_rate: u64,
        /// whether the mix should spatially place users, default=false
        spatial_audio: bool,
        /// whether the ssrc-audio-level RTP extension must be negotiated for new joins, default=true
        audiolevel_ext: bool,
        /// whether to emit event to other users or not
        audiolevel_event: bool,
        /// number of packets with audio level (default=100, 2 seconds)
        audio_active_packets: u64,
        /// percent of packets we expect participants may miss,
        /// to help with FEC (default=0, max=20; automatically used for forwarders too)
        default_expectedloss: u64,
        /// bitrate in bps to use for the all participants
        /// (default=0, which means libopus decides; automatically used for forwarders too)
        default_bitrate: u64,
        /// whether to record the room or not, default=false
        record: bool,
        /// /path/to/the/recording.wav
        record_file: String,
        /// /path/to/; makes record_file a relative path, if provided
        record_dir: String,
        /// whether all participants in the room should be individually recorded to mjr files, default=false
        mjrs: bool,
        /// /path/to/
        mjrs_dir: String,
        /// whether participants should be allowed to join via plain RTP as well, default=false
        allow_rtp_participants: bool,
        /// non-hierarchical array of string group names to use to gat participants,
        /// for external forwarding purposes only
        groups: Vec<String>
    }
);

make_dto!(
    AudioBridgeEditParams,
    required { room: JanusId },
    optional {
        /// room secret, mandatory if configured
        secret: String,
        /// new pretty name of the room
        new_description: String,
        /// new password required to edit/destroy the room
        new_secret: String,
        /// new PIN required to join the room, PIN will be removed if set to an empty string
        new_pin: String,
        /// whether the room should appear in a list request
        new_is_private: bool,
        /// new path where new recording files should be saved
        new_record_dir: String,
        /// new path where new MJR files should be saved
        new_mjrs_dir: String,
        /// whether the room should be also removed from the config file, default=false
        permanent: bool
    }
);

make_dto!(
    AudioBridgeDestroyParams,
    required { room: JanusId },
    optional {
        /// room secret, mandatory if configured
        secret: String,
        /// whether the room should be also removed from the config file, default=false
        permanent: bool
    }
);

make_dto!(
    AudioBridgeEnableRecordingParams,
    required { room: JanusId },
    optional {
        /// room secret; mandatory if configured
        secret: String,
        /// whether this room should be automatically recorded or not
        record: bool,
        /// file where audio recording should be saved
        record_file: String,
        /// path where audio recording file should be saved
        record_dir: String,
    }
);

make_dto!(
    AudioBridgeEnableMjrsParams,
    required { room: JanusId },
    optional {
        /// room secret; mandatory if configured
        secret: String,
        /// whether all participants in the room should be individually recorded to mjr files or not
        mjrs: bool,
        /// path where all MJR files should be saved to
        mjrs_dir: String,
    }
);

make_dto!(AudioBridgeExistsParams, required { room: JanusId });

make_dto!(
    AudioBridgeAllowedParams,
    required {
        room: JanusId,
        /// Array of strings (tokens users might pass in "join", only for add|remove)
        action: AudioBridgeAllowAction,
        allowed: Vec<String>
    },
    optional {
        /// Room secret, mandatory if configured
        secret: String
    }
);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioBridgeAllowAction {
    Enable,
    Disable,
    Add,
    Remove,
}

make_dto!(
    AudioBridgeKickParams,
    required {
        /// Participant ID
        id: JanusId,
        room: JanusId
    },
    optional {
        /// Room secret, mandatory if configured
        secret: String
    }
);

make_dto!(
    AudioBridgeKickAllParams,
    required { room: JanusId },
    optional {
        /// Room secret, mandatory if configured
        secret: String
    }
);

make_dto!(
    AudioBridgeListParticipantsParams,
    required { room: JanusId }
);

make_dto!(
    AudioBridgeJoinParams,
    required { room: JanusId },
    optional {
        /// Unique ID to assign to the participant, assigned by the plugin if missing
        id: JanusId,
        /// Group to assign to this participant (for forwarding purposes only, mandatory if enabled in the room)
        group: String,
        /// Password required to join the room, if any.
        pin: String,
        /// Display name to have in the room
        display: String,
        /// Invitation token, in case the room has an ACL.
        token: String,
        /// Whether to start unmuted or muted, false by default
        muted: bool,
        /// Whether to start suspended or not, false by default
        suspended: bool,
        /// Whether room events should be paused, if the user is joining as suspended, false by default
        pause_events: bool,
        /// Codec to use, among opus (default), pcma (A-Law) or pcmu (mu-Law)"
        codec: AudioBridgeCodec,
        /// Bitrate to use for the Opus stream in bps, default=0 (libopus decides)
        bitrate: u64,
        /// 0-10, Opus-related complexity to use, the higher the value, the better the quality (but more CPU); default is 4
        quality: u8,
        /// 0-20, a percentage of the expected loss (capped at 20%), only needed in case FEC is used,
        /// default is 0 (FEC disabled even when negotiated) or the room default
        expected_loss: u8,
        /// Percent value, <100 reduces volume, >100 increases volume, default is 100 (no volume change)
        volume: u64,
        /// In case spatial audio is enabled for the room, panning of this participant (0=left, 50=center, 100=right)
        spatial_position: u8,
        /// Room management password, if provided the user is an admin and can't be globally muted with `mute_room`
        secret: String,
        /// If provided, overrides the room `audio_level_average` for this user.
        audio_level_average: u64,
        /// If provided, overrides the room audio_active_packets for this user.
        audio_active_packets: u64,
        /// Whether to record this user's contribution to a .mjr file (mixer not involved)
        record: bool,
        /// Basename of the file to record to, -audio.mjr will be added by the plugin; will be relative to mjrs_dir,
        /// if configured in the room
        filename: String,
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
        /// As a policy, plugins in Janus tend to enforce the same negotiation pattern used to setup the PeerConnection i
        /// nitially for renegotiations too, as it reduces the risk of issues like glare: this means that users will NOT be
        /// able to send an SDP offer to the AudioBridge plugin to update an existing PeerConnection, if that PeerConnection
        /// had previously been originated by a plugin offer instead. The plugin will treat this as an error.
        generate_offer: bool,
        /// Notice that, while the AudioBridge assumes participants will exchange media via WebRTC, there's a less known
        /// feature that allows you to use plain RTP to join an AudioBridge room instead. This functionality may be helpful
        /// in case you want, e.g., SIP based endpoints to join an AudioBridge room, by crafting SDPs for the SIP dialogs
        /// yourself using the info exchanged with the plugin. In order to do that, you keep on using the API to join as a
        /// participant as explained above, but instead of negotiating a PeerConnection as you usually would
        rtp: AudioBridgeRTP,
    }
);

make_dto!(
    AudioBridgeRTP,
    required {
        /// IP address you want media to be sent to
        ip: String,
        /// port you want media to be sent to
        port: u16
    },
    optional {
        /// payload type to use for RTP packets (only needed in case Opus is used, automatic for G.711)
        payload_type: String,
        /// ID of the audiolevel RTP extension, if used
        audiolevel_ext: String,
        /// whether FEC should be enabled for the Opus stream (only needed in case Opus is used)
        fec: bool
    }
);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioBridgeCodec {
    Opus,
    /// A-Law
    Pcma,
    /// mu-Law
    Pcmu,
}

make_dto!(
    AudioBridgeConfigureParams,
    optional {
        /// whether to unmute or mute
        muted: bool,
        /// new display name to have in the room (see "join" for more info)
        display: String,
        /// new bitrate to use for the Opus stream
        bitrate: u64,
        /// new Opus-related complexity to use (see "join" for more info)
        quality: u8,
        /// new value for the expected loss (see "join" for more info)
        expected_loss: u8,
        /// new volume percent value (see "join" for more info)
        volume: u64,
        /// in case spatial audio is enabled for the room, new panning of this participant (0=left, 50=center, 100=right)
        spatial_position: u8,
        /// whether denoising via RNNoise should be performed for this participant (default=room value)
        denoise: bool,
        /// whether to record this user's contribution to a .mjr file (mixer not involved)
        record: bool,
        /// basename of the file to record to, -audio.mjr will be added by the plugin; will be relative to mjrs_dir
        /// if configured in the room
        filename: String,
        /// new group to assign to this participant, if enabled in the room (for forwarding purposes)
        group: String
    }
);

make_dto!(
    AudioBridgeMuteParams,
    required {
        /// Participant ID
        id: JanusId,
        /// Room ID
        room: JanusId
    },
    optional {
        /// Room secret, mandatory if configured
        secret: String
    }
);

make_dto!(
    AudioBridgeMuteRoomParams,
    required { room: JanusId },
    optional {
        /// Room secret, mandatory if configured
        secret: String
    }
);

make_dto!(
    AudioBridgeChangeRoomParams,
    required { room: JanusId },
    optional {
        /// numeric ID of the room to move to
        id: JanusId,
        /// Password required to join the room, if any.
        pin: String,
        /// Group to assign to this participant (for forwarding purposes only, mandatory if enabled in the room)
        group: String,
        /// new display name to have in the room (see "join" for more info)
        display: String,
        /// invitation token, in case the new room has an ACL
        token: String,
        /// whether to unmute or mute
        muted: bool,
        /// whether to start suspended or not
        suspend: bool,
        /// whether room events should be paused, if the user is joining as suspended, false by default
        pause_events: bool,
        /// new bitrate to use for the Opus stream
        bitrate: u64,
        /// new Opus-related complexity to use (see "join" for more info)
        quality: u8,
        /// new value for the expected loss (see "join" for more info)
        expected_loss: u8,
        /// new volume percent value (see "join" for more info)
        volume: u64,
        /// in case spatial audio is enabled for the room, new panning of this participant (0=left, 50=center, 100=right)
        spatial_position: u8,
        /// whether denoising via RNNoise should be performed for this participant (default=room value)
        denoise: bool
    }
);
