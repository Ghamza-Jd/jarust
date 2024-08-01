use serde::Serialize;

//
// Create Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomCreateOptions {
    /// unique numeric ID, chosen by plugin if missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<u64>,

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

    /// whether subscriptions are required to provide a valid private_id to associate with a publisher, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_pvtid: Option<bool>,

    /// whether access to the room requires signed tokens; default=false, only works if signed tokens are used in the core as well
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_tokens: Option<bool>,

    /// max number of concurrent senders (e.g., 6 for a video conference or 1 for a webinar, default=3)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishers: Option<u64>,

    /// max video bitrate for senders (e.g., 128000)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,

    /// whether the above cap should act as a limit to dynamic bitrate changes by publishers, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate_cap: Option<bool>,

    /// send a FIR to publishers every fir_freq seconds (0=disable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fir_freq: Option<u64>,

    /// audio codec to force on publishers, default=opus
    /// can be a comma separated list in order of preference, e.g., `opus,pcmu`
    /// opus|g722|pcmu|pcma|isac32|isac16
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiocodec: Option<String>,

    /// video codec to force on publishers, default=vp8
    /// can be a comma separated list in order of preference, e.g., `vp9,vp8,h264`
    /// vp8|vp9|h264|av1|h265
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videocodec: Option<String>,

    /// VP9-specific profile to prefer (e.g., "2" for "profile-id=2")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp9_profile: Option<String>,

    /// H.264-specific profile to prefer (e.g., "42e01f" for "profile-level-id=42e01f")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h264_profile: Option<String>,

    /// whether inband FEC must be negotiated; only works for Opus, default=true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opus_fec: Option<bool>,

    /// whether DTX must be negotiated; only works for Opus, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opus_dtx: Option<bool>,

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

    /// whether the video-orientation RTP extension must be negotiated/used or not for new publishers, default=true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videoorient_ext: Option<bool>,

    /// whether the playout-delay RTP extension must be negotiated/used or not for new publishers, default=true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playoutdelay_ext: Option<bool>,

    /// whether the transport wide CC RTP extension must be negotiated/used or not for new publishers, default=true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_wide_cc_ext: Option<bool>,

    /// whether to record the room or not, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<bool>,

    /// folder where recordings should be stored, when enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_dir: Option<String>,

    /// whether recording can only be started/stopped if the secret is provided, or using the global enable_recording request, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_record: Option<bool>,

    /// optional, whether to notify all participants when a new participant joins the room. default=false
    /// The Videoroom plugin by design only notifies new feeds (publishers), and enabling this may result in extra notification traffic.
    /// This flag is particularly useful when enabled with `require_pvtid` for admin to manage listening-only participants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_joining: Option<bool>,

    /// whether all participants are required to publish and subscribe using end-to-end media encryption, e.g., via Insertable Streams; default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_e2ee: Option<bool>,

    /// whether a dummy publisher should be created in this room, with one separate m-line for each codec supported in the room;
    /// this is useful when there's a need to create subscriptions with placeholders for some or all m-lines, even when they aren't used yet; default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dummy_publisher: Option<bool>,

    /// in case `dummy_publisher` is set to `true`, array of codecs to offer, optionally with a fmtp attribute to match (codec/fmtp properties).
    /// If not provided, all codecs enabled in the room are offered, with no fmtp. Notice that the fmtp is parsed, and only a few codecs are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dummy_streams: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoRoomAudioCodec {
    OPUS,
    G722,
    PCMU,
    PCMA,
    ISAC32,
    ISAC16,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoRoomVideoCodec {
    VP8,
    VP9,
    H264,
    AV1,
    H265,
}

//
// Edit message
//

#[derive(Serialize, Default)]
pub struct VideoRoomEditOptions {
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

    /// whether the room should require `private_id` from subscribers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_require_pvtid: Option<bool>,

    /// new bitrate cap to force on all publishers (except those with custom overrides)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_bitrate: Option<u64>,

    /// new period for regular PLI keyframe requests to publishers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_fir_freq: Option<u64>,

    /// new cap on the number of concurrent active WebRTC publishers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_publishers: Option<u64>,

    /// whether recording state can only be changed when providing the room secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_lock_record: Option<bool>,

    /// the new path where the next .mjr files should be saved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_rec_dir: Option<String>,

    /// whether the room should be also removed from the config file, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

//
// Destroy Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomDestroyOptions {
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// whether the room should be also removed from the config file, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

//
// Join Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomPublisherJoinOptions {
    /// unique ID to register for the publisher;
    /// optional, will be chosen by the plugin if missing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    /// display name for the publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    /// invitation token, in case the room has an ACL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Serialize, Default)]
pub struct VideoRoomSubscriberJoinOptions {
    /// whether subscriptions should include a msid that references the publisher; false by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_msid: Option<bool>,

    /// whether a new SDP offer is sent automatically when a subscribed publisher leaves; true by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoupdate: Option<bool>,

    /// unique ID of the publisher that originated this request;
    /// optional, unless mandated by the room configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_id: Option<u64>,

    /// list of media streams to subscribe to
    streams: Vec<VideoRoomSubscriberJoinStream>,
}

#[derive(Serialize, Default)]
pub struct VideoRoomSubscriberJoinStream {
    /// unique ID of publisher owning the stream to subscribe to
    pub feed: u64,

    /// unique mid of the publisher stream to subscribe to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid: Option<u64>,

    /// id to map this subscription with entries in streams list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossrefid: Option<u64>,
}

//
// Allowed Message
//

#[derive(PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoRoomAllowedAction {
    Enable,
    Disable,
    Add,
    Remove,
}

#[derive(Serialize, Default)]
pub struct VideoRoomAllowedOptions {
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

//
// Kick Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomKickOptions {
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

//
// Moderate Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomModerateOptions {
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

//
// Enable Recording Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomEnableRecordingOptions {
    /// whether participants in this room should be automatically recorded or not
    pub record: bool,
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

//
// List Forwarders Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomListForwardersOptions {
    /// room secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

//
// Configure Publisher Message
//

#[derive(Serialize, Default)]
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
    streams: Vec<VideoRoomConfigurePublisherStream>,

    /// descriptions (names) for the published streams
    #[serde(skip_serializing_if = "Vec::is_empty")]
    descriptions: Vec<VideoRoomPublishDescription>,
}

#[derive(Serialize, Default)]
pub struct VideoRoomConfigurePublisherStream {
    pub mid: u64,

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

//
// Publish Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomPublishOptions {
    /// audio codec to prefer among the negotiated ones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiocodec: Option<String>,

    /// video codec to prefer among the negotiated ones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videocodec: Option<String>,

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

    /// if provided, overrided the room audio_level_average for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_level_average: Option<u64>,

    /// if provided, overrided the room audio_active_packets for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_active_packets: Option<u64>,

    /// descriptions (names) for the published streams
    #[serde(skip_serializing_if = "Vec::is_empty")]
    descriptions: Vec<VideoRoomPublishDescription>,
}

#[derive(Serialize, Default)]
pub struct VideoRoomPublishDescription {
    /// unique mid of a stream being published
    pub mid: u64,

    /// text description of the stream (e.g., My front webcam)
    pub description: String,
}

//
// Configure Subscriber Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomConfigureSubscriberOptions {
    /// list of streams to configure
    streams: Vec<VideoRoomConfigureSubscriberStream>,
    /// trigger an ICE restart
    #[serde(skip_serializing_if = "Option::is_none")]
    restart: Option<bool>,
}

#[derive(Serialize, Default)]
pub struct VideoRoomConfigureSubscriberStream {
    /// mid of the m-line to refer to
    pub mid: u64,

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

//
// Switch Message
//

#[derive(Serialize, Default)]
pub struct VideoRoomSwitchStream {
    /// unique ID of the publisher the new source is from
    pub feed: u64,

    /// unique mid of the source we want to switch to
    pub mid: u64,

    /// unique mid of the stream we want to pipe the new source to
    pub sub_mid: u64,
}
