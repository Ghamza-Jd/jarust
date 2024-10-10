use crate::JanusId;
use serde::Serialize;

impl_tryfrom_serde_value!(
    VideoRoomDestroyOptions VideoRoomAllowedOptions VideoRoomAllowedAction
    VideoRoomKickOptions VideoRoomEnableRecordingOptions VideoRoomPublisherJoinOptions VideoRoomSubscriberJoinOptions
    VideoRoomConfigurePublisherOptions VideoRoomConfigureSubscriberOptions JoinAndConfigureOptions VideoRoomPublishOptions
);

make_dto!(
    VideoRoomCreateParams,
    optional {
        /// Room ID, chosen by plugin if missing
        room: JanusId,
        /// whether the room should be saved in the config file, default=false
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
        /// whether subscriptions are required to provide a valid private_id to associate with a publisher, default=false
        require_pvtid: bool,
        /// whether access to the room requires signed tokens; default=false, only works if signed tokens are used in the core as well
        signed_tokens: bool,
        /// max number of concurrent senders (e.g., 6 for a video conference or 1 for a webinar, default=3)
        publishers: u64,
        /// max video bitrate for senders (e.g., 128000)
        bitrate: u64,
        /// whether the above cap should act as a limit to dynamic bitrate changes by publishers, default=false
        bitrate_cap: bool,
        /// send a FIR to publishers every fir_freq seconds (0=disable)
        fir_freq: u64,
        /// audio codec to force on publishers, default=opus
        /// can be a comma separated list in order of preference, e.g., `opus,pcmu`
        /// opus|g722|pcmu|pcma|isac32|isac16
        audiocodec: String,
        /// video codec to force on publishers, default=vp8
        /// can be a comma separated list in order of preference, e.g., `vp9,vp8,h264`
        /// vp8|vp9|h264|av1|h265
        videocodec: String,
        /// VP9-specific profile to prefer (e.g., "2" for "profile-id=2")
        vp9_profile: String,
        /// H.264-specific profile to prefer (e.g., "42e01f" for "profile-level-id=42e01f")
        h264_profile: String,
        /// whether inband FEC must be negotiated; only works for Opus, default=true
        opus_fec: bool,
        /// whether DTX must be negotiated; only works for Opus, default=false
        opus_dtx: bool,
        /// whether the ssrc-audio-level RTP extension must be negotiated for new joins, default=true
        audiolevel_ext: bool,
        /// whether to emit event to other users or not
        audiolevel_event: bool,
        /// number of packets with audio level (default=100, 2 seconds)
        audio_active_packets: u64,
        /// average value of audio level (127=muted, 0='too loud', default=25)
        audio_level_average: u64,
        /// whether the video-orientation RTP extension must be negotiated/used or not for new publishers, default=true
        videoorient_ext: bool,
        /// whether the playout-delay RTP extension must be negotiated/used or not for new publishers, default=true
        playoutdelay_ext: bool,
        /// whether the transport wide CC RTP extension must be negotiated/used or not for new publishers, default=true
        transport_wide_cc_ext: bool,
        /// whether to record the room or not, default=false
        record: bool,
        /// folder where recordings should be stored, when enabled
        record_dir: String,
        /// whether recording can only be started/stopped if the secret is provided, or using the global enable_recording request, default=false
        lock_record: bool,
        /// optional, whether to notify all participants when a new participant joins the room. default=false
        /// The Videoroom plugin by design only notifies new feeds (publishers), and enabling this may result in extra notification traffic.
        /// This flag is particularly useful when enabled with `require_pvtid` for admin to manage listening-only participants.
        notify_joining: bool,
        /// whether all participants are required to publish and subscribe using end-to-end media encryption, e.g., via Insertable Streams; default=false
        require_e2ee: bool,
        /// whether a dummy publisher should be created in this room, with one separate m-line for each codec supported in the room;
        /// this is useful when there's a need to create subscriptions with placeholders for some or all m-lines, even when they aren't used yet; default=false
        dummy_publisher: bool,
        /// in case `dummy_publisher` is set to `true`, array of codecs to offer, optionally with a fmtp attribute to match (codec/fmtp properties).
        /// If not provided, all codecs enabled in the room are offered, with no fmtp. Notice that the fmtp is parsed, and only a few codecs are supported.
        dummy_streams: bool,
    }
);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoRoomAudioCodec {
    OPUS,
    G722,
    PCMU,
    PCMA,
    ISAC32,
    ISAC16,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoRoomVideoCodec {
    VP8,
    VP9,
    H264,
    AV1,
    H265,
}

make_dto!(
    VideoRoomEditParams,
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
        /// whether the room should require `private_id` from subscribers
        new_require_pvtid: bool,
        /// new bitrate cap to force on all publishers (except those with custom overrides)
        new_bitrate: u64,
        /// new period for regular PLI keyframe requests to publishers
        new_fir_freq: u64,
        /// new cap on the number of concurrent active WebRTC publishers
        new_publishers: u64,
        /// whether recording state can only be changed when providing the room secret
        new_lock_record: bool,
        /// the new path where the next .mjr files should be saved
        new_rec_dir: String,
        /// whether the room should be also removed from the config file, default=false
        permanent: bool,
    }
);

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomDestroyOptions {
    pub room: JanusId,

    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// whether the room should be also removed from the config file, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomPublisherJoinOptions {
    pub room: JanusId,
    /// unique ID to register for the publisher;
    /// optional, will be chosen by the plugin if missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<JanusId>,

    /// display name for the publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// invitation token, in case the room has an ACL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomSubscriberJoinOptions {
    pub room: JanusId,
    /// whether subscriptions should include a msid that references the publisher; false by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_msid: Option<bool>,

    /// whether a new SDP offer is sent automatically when a subscribed publisher leaves; true by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoupdate: Option<bool>,

    /// unique ID of the publisher that originated this request;
    /// optional, unless mandated by the room configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_id: Option<JanusId>,

    /// list of media streams to subscribe to
    pub streams: Vec<VideoRoomSubscriberJoinStream>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomSubscriberJoinStream {
    /// unique ID of publisher owning the stream to subscribe to
    pub feed: JanusId,

    /// unique mid of the publisher stream to subscribe to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid: Option<String>,

    /// id to map this subscription with entries in streams list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossrefid: Option<u64>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct VideoRoomSubscriberUnsubscribeStream {
    /// unique ID of publisher owning the stream to subscribe to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<JanusId>,

    /// unique mid of the publisher stream to subscribe to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid: Option<String>,

    /// id to map this subscription with entries in streams list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mid: Option<u64>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoRoomAllowedAction {
    Enable,
    Disable,
    Add,
    Remove,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomAllowedOptions {
    pub room: JanusId,
    pub action: VideoRoomAllowedAction,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed: Vec<String>,
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomKickOptions {
    pub room: JanusId,
    pub participant: JanusId,
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomModerateOptions {
    pub room: JanusId,
    pub participant: JanusId,
    pub m_line: u64,
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomEnableRecordingOptions {
    pub room: JanusId,
    /// whether participants in this room should be automatically recorded or not
    pub record: bool,
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomListForwardersOptions {
    room: JanusId,
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomConfigurePublisherOptions {
    /// bitrate cap to return via REMB;
    /// overrides the global room value if present (unless `bitrate_cap` is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,

    /// whether we should send this publisher a keyframe request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframe: Option<bool>,

    /// whether this publisher should be recorded or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,

    /// if recording, the base path/file to use for the recording files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// new display name to use in the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// new `audio_active_packets` to overwrite in the room one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_active_packets: Option<u64>,

    /// new `audio_level_average` to overwrite the room one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_level_average: Option<u64>,

    /// list of streams to configure
    pub streams: Vec<VideoRoomConfigurePublisherStream>,

    /// descriptions (names) for the published streams
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub descriptions: Vec<VideoRoomPublishDescription>,
}

// TODO: Check if it's okay to remove default
#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct VideoRoomConfigurePublisherStream {
    pub mid: String,

    /// whether we should send this publisher a keyframe request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyframe: Option<bool>,

    /// depending on whether the media addressed by the above mid should be relayed or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send: Option<bool>,

    /// minimum delay to enforce via the playout-delay RTP extension, in blocks of 10ms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_delay: Option<u64>,

    /// maximum delay to enforce via the playout-delay RTP extension, in blocks of 10ms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_delay: Option<u64>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct JoinAndConfigureOptions {
    #[serde(flatten)]
    pub join_options: VideoRoomPublisherJoinOptions,
    #[serde(flatten)]
    pub configure_options: VideoRoomConfigurePublisherOptions,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct VideoRoomPublishOptions {
    /// audio codec to prefer among the negotiated ones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiocodec: Option<VideoRoomAudioCodec>,

    /// video codec to prefer among the negotiated ones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videocodec: Option<VideoRoomVideoCodec>,

    /// bitrate cap to return via REMB
    /// overrides the global room value if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,

    /// whether this publisher should be recorded or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,

    /// if recording, the base path/file to use for the recording files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// display name to use in the room
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// if provided, override the room audio_level_average for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_level_average: Option<u64>,

    /// if provided, override the room audio_active_packets for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_active_packets: Option<u64>,

    /// descriptions (names) for the published streams
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub descriptions: Vec<VideoRoomPublishDescription>,
}

// TODO: Check if it's okay to remove default
#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct VideoRoomPublishDescription {
    /// unique mid of a stream being published
    pub mid: String,

    /// text description of the stream (e.g., My front webcam)
    pub description: String,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct VideoRoomConfigureSubscriberOptions {
    /// list of streams to configure
    pub streams: Vec<VideoRoomConfigureSubscriberStream>,
    /// trigger an ICE restart
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,
}

// TODO: Check if it's okay to remove default
#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct VideoRoomConfigureSubscriberStream {
    /// mid of the m-line to refer to
    pub mid: String,

    /// depending on whether the mindex media should be relayed or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send: Option<bool>,

    /// substream to receive (0-2), in case simulcasting is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substream: Option<u64>,

    /// temporal layers to receive (0-2), in case simulcasting is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal: Option<u64>,

    /// How much time (in us, default 250000) without receiving packets will make us drop to the substream below
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<u64>,

    /// spatial layer to receive (0-2), in case SVC is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacial_layer: Option<u64>,

    /// temporal layers to receive (0-2), in case SVC is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_layer: Option<u64>,

    /// if provided, overrides the room `audio_level_average` for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_level_average: Option<u64>,

    /// if provided, overrides the room `audio_active_packets` for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_active_packets: Option<u64>,

    /// minimum delay to enforce via the playout-delay RTP extension, in blocks of 10ms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_delay: Option<u64>,

    /// maximum delay to enforce via the playout-delay RTP extension, in blocks of 10ms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_delay: Option<u64>,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomSwitchStream {
    /// unique ID of the publisher the new source is from
    pub feed: JanusId,

    /// unique mid of the source we want to switch to
    pub mid: String,

    /// unique mid of the stream we want to pipe the new source to
    pub sub_mid: String,
}

#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct VideoRoomRtpForwardOptions {
    pub room: JanusId,
    /// unique numeric ID of the publisher to relay externally
    pub publisher_id: JanusId,

    /// host address to forward the RTP and data packets to
    pub host: String,

    /// ipv4|ipv6, if we need to resolve the host address to an IP; by default, whatever we get
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_family: Option<String>,

    pub streams: Vec<VideoRoomRtpForwardStream>,

    /// length of authentication tag (32 or 80); optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srtp_suite: Option<u16>,

    /// key to use as crypto (base64 encoded key as in SDES); optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srtp_crypto: Option<String>,
}

// TODO: Check if it's okay to remove default
#[cfg_attr(feature = "option_builder", derive(bon::Builder))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize)]
pub struct VideoRoomRtpForwardStream {
    /// mid of publisher stream to forward
    pub mid: String,

    /// host address to forward the packets to; optional, will use global one if missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// optional, will use global one if missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_family: Option<String>,

    /// port to forward the packets to
    pub port: u16,

    /// SSRC to use when forwarding; optional, and only for RTP streams, not data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssrc: Option<String>,

    /// payload type to use when forwarding; optional, and only for RTP streams, not data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<String>,

    /// port to contact to receive RTCP feedback from the recipient; optional, and only for RTP streams, not data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtcp_port: Option<u16>,

    /// set to true if the source is simulcast and you want the forwarder to act as a regular viewer
    /// (single stream being forwarded) or false otherwise (substreams forwarded separately); optional, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulcast: Option<bool>,

    /// if video and simulcasting, port to forward the packets from the second substream/layer to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_2: Option<u16>,

    /// if video and simulcasting, SSRC to use the second substream/layer; optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssrc_2: Option<String>,

    /// if video and simulcasting, payload type to use the second substream/layer; optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt_2: Option<String>,

    /// if video and simulcasting, port to forward the packets from the third substream/layer to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_3: Option<u16>,

    /// if video and simulcasting, SSRC to use the third substream/layer; optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssrc_3: Option<String>,

    /// if video and simulcasting, payload type to use the third substream/layer; optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt_3: Option<String>,
}
