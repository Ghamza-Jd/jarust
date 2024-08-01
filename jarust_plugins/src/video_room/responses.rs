use serde::Deserialize;

use crate::Identifier;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct Room {
    /// unique numeric ID
    pub room: Identifier,
    /// name of the room
    pub description: String,
    /// whether a PIN is required to join this room
    pub pin_required: bool,
    /// whether this room is 'private' (as in hidden) or not
    pub is_private: bool,
    /// how many publishers can actually publish via WebRTC at the same time
    pub max_publishers: u64,
    /// bitrate cap that should be forced (via REMB) on all publishers by default
    pub bitrate: u64,
    /// whether the above cap should act as a limit to dynamic bitrate changes by publishers
    pub bitrate_cap: Option<bool>,
    /// how often a keyframe request is sent via PLI/FIR to active publishers
    pub fir_freq: u64,
    /// whether subscriptions in this room require a private_id
    pub require_pvtid: bool,
    /// whether end-to-end encrypted publishers are required
    pub require_e2ee: bool,
    /// whether a dummy publisher exists for placeholder subscriptions
    pub dummy_publisher: bool,
    /// whether an event is sent to notify all participants if a new participant joins the room
    pub notify_joining: bool,
    /// comma separated list of allowed audio codecs
    pub audiocodec: String,
    /// comma separated list of allowed video codecs
    pub videocodec: String,
    /// whether inband FEC must be negotiated (note: only available for Opus)
    pub opus_fec: Option<bool>,
    /// whether DTX must be negotiated (note: only available for Opus)
    pub opus_dtx: Option<bool>,
    /// whether the room is being recorded
    pub record: bool,
    /// if recording, the path where the .mjr files are being saved
    pub rec_dir: Option<String>,
    /// whether the room recording state can only be changed providing the secret
    pub lock_record: bool,
    /// count of the participants (publishers, active or not; not subscribers)
    pub num_participants: u64,
    /// whether the ssrc-audio-level extension must be negotiated or not for new publishers
    pub audiolevel_ext: bool,
    /// whether to emit event to other users about audiolevel
    pub audiolevel_event: bool,
    /// number of packets with audio level for checkup (optional, only if audiolevel_event is true)
    pub audio_active_packets: Option<u64>,
    /// average audio level (optional, only if audiolevel_event is true)
    pub audio_level_average: Option<u64>,
    /// whether the video-orientation extension must be negotiated or not for new publishers
    pub videoorient_ext: bool,
    /// whether the playout-delay extension must be negotiated or not for new publishers
    pub playoutdelay_ext: bool,
    /// whether the transport wide cc extension must be negotiated or not for new publishers
    pub transport_wide_cc_ext: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct Participant {
    /// unique numeric ID of the participant
    pub id: Identifier,
    /// display name of the participant, if any
    pub display: Option<String>,
    /// whether user is an active publisher in the room
    pub publisher: bool,
    /// whether user is talking or not (only if audio levels are used)
    pub talking: Option<bool>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct Publisher {
    /// unique ID of active publisher
    pub id: Identifier,
    /// display name of active publisher
    pub display: Option<String>,
    /// true if this participant is a dummy publisher
    pub dummy: bool,
    pub streams: Vec<Stream>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct Stream {
    /// type of published stream (audio|video|data)
    #[serde(rename = "type")]
    pub media_type: String,

    /// unique mindex of published stream
    pub mindex: u64,

    /// unique mid of published stream
    pub mid: u64,

    /// if true, it means this stream is currently inactive/disabled
    /// (and so codec, description, etc. will be missing)
    pub disabled: bool,

    /// codec used for published stream
    pub codec: Option<String>,

    /// text description of published stream, if any
    pub description: Option<String>,

    /// true if this stream audio has been moderated for this participant
    pub moderated: Option<bool>,

    /// true if published stream uses simulcast
    pub simulcast: Option<bool>,

    /// true if published stream uses SVC (VP9 and AV1 only)
    pub svc: Option<bool>,

    /// whether the publisher stream has audio activity or not (only if audio levels are used)
    pub talking: Option<bool>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct Attendee {
    /// unique ID of the attendee
    pub id: Identifier,

    /// display name of the attendee, if any
    pub display: Option<String>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RtpForwarder {
    /// unique numeric ID assigned to this forwarder
    pub stream_id: u64,
    /// type of media (audio|video|data)
    #[serde(rename = "type")]
    pub media_type: String,
    /// host this forwarder is streaming to, same as request if not resolved
    pub host: String,
    /// port this forwarder is streaming to, same as request if configured
    pub port: u64,
    /// local port this forwarder is using to get RTCP feedback, if any
    pub local_rtcp_port: Option<u64>,
    /// remote port this forwarder is getting RTCP feedback from, if any
    pub remote_rtmp_port: Option<u64>,
    /// SSRC this forwarder is using, same as request if configured
    pub ssrc: Option<String>,
    /// payload type this forwarder is using, same as request if configured
    pub pt: Option<String>,
    /// video substream this video forwarder is relaying
    pub substream: Option<u64>,
    /// whether the RTP stream is encrypted (not used for data)
    pub strp: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RtpForwarderPublisher {
    pub publisher_id: Identifier,
    pub forwarders: Vec<RtpForwarder>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomCreatedRsp {
    pub room: Identifier,
    pub permanent: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ListRoomsRsp {
    pub list: Vec<Room>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ListParticipantsRsp {
    pub room: Identifier,
    pub participants: Vec<Participant>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomDestroyedRsp {
    pub room: Identifier,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomEditedRsp {
    pub room: Identifier,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RoomExistsRsp {
    pub room: Identifier,
    pub exists: bool,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AccessRsp {
    pub room: Identifier,
    // TODO: Is it better to have an empty Vec here or should this be wrapped in Option?
    #[serde(default = "Vec::default")]
    pub allowed: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ListForwardersRsp {
    pub room: Identifier,
    pub publisher: Vec<RtpForwarderPublisher>,
}
