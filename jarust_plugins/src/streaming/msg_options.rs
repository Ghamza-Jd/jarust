use crate::JanusId;
use serde::Serialize;

impl_tryfrom_serde_value!(
    StreamingCreateOptions StreamingDestroyOptions StreamingInfoOptions
);

//
// Create Message
// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L3311-L4175
// TODO: only RTP type is supported
//

#[derive(Serialize, Default)]
pub struct StreamingCreateOptions {
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

    /// <rtp|live|ondemand|rtsp>
    #[serde(rename = "type")]
    pub mountpoint_type: String,

    // RTP only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<StreamingRtpMedia>>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamingMountpointType {
    RTP,
    LIVE,
    ONDEMAND,
    RTSP,
}

// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L1100
#[derive(Serialize, Default)]
pub struct StreamingRtpMedia {
    /// audio|video|data
    #[serde(rename = "type")]
    pub media_type: String,

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

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamingRtpMediaType {
    AUDIO,
    VIDEO,
    DATA,
}

//
// Destroy Message
//

#[derive(Serialize, Default)]
pub struct StreamingDestroyOptions {
    /// mountpoint secret, mandatory if configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    /// whether the mountpoint should be also removed from the config file, default=false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

//
// Info Message
//

#[derive(Serialize, Default)]
pub struct StreamingInfoOptions {
    /// mountpoint secret, mandatory if configured to access sensitive info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
