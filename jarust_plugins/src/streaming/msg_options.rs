use crate::JanusId;
use serde::Serialize;

impl_tryfrom_serde_value!(
    StreamingCreateOptions StreamingDestroyOptions
);

//
// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L3311-L4175
// TODO: only RTP type is supported
//
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct StreamingCreateOptions {
    #[serde(rename = "type")]
    pub mountpoint_type: StreamingMountpointType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<JanusId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// pin required for viewers to access mountpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,

    /// whether the mountpoint should be saved to the configuration file or not, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,

    // RTP only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<StreamingRtpMedia>>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum StreamingMountpointType {
    RTP,
    LIVE,
    ONDEMAND,
    RTSP,
}

// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L1100
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct StreamingRtpMedia {
    /// audio|video|data
    #[serde(rename = "type")]
    pub media_type: StreamingRtpMediaType,

    /// Unique mid to assign to this stream in negociated PeerConnections
    pub mid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcast: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iface: Option<String>,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtcpport: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<u8>, // payload type is restricted to 0-127
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmtp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skew: Option<bool>,
    // missing video only and data only parameters ?
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum StreamingRtpMediaType {
    AUDIO,
    VIDEO,
    DATA,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct StreamingDestroyOptions {
    #[serde(rename = "id")]
    pub mountpoint: JanusId,

    /// mountpoint secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// whether the mountpoint should be also removed from the config file, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}
