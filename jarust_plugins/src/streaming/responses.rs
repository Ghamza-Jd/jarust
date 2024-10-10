use crate::JanusId;
use serde::Deserialize;

// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L4335-L4414
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct MountpointCreatedRsp {
    pub created: String,
    pub permanent: bool,
    pub stream: MountpointCreated,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct MountpointCreated {
    pub id: JanusId,
    /// <live|on demand>
    #[serde(rename = "type")]
    pub mountpoint_type: String,
    pub description: String,
    pub is_private: bool,
    // RTP only
    pub host: Option<String>,
    pub ports: Option<Vec<RtpMediaCreated>>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RtpMediaCreated {
    /// <audio|video|data>
    #[serde(rename = "type")]
    pub media_type: String,
    pub mid: String,
    pub msid: Option<String>,
    pub port: Option<u16>,
    pub rtcp_port: Option<u16>,
    pub port_2: Option<u16>,
    pub port_3: Option<u16>,
}

// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L4994-L4997
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct MountpointDestroyedRsp {
    pub destroyed: JanusId,
}

// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L3058-L3127
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct ListMountpointsRsp {
    pub list: Vec<MountpointListed>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct MountpointListed {
    pub id: JanusId,
    /// <live|on demand>
    #[serde(rename = "type")]
    pub mountpoint_type: String,
    pub description: String,
    pub metadata: Option<String>,
    pub enabled: bool,
    pub media: Option<Vec<RtpMediaListed>>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RtpMediaListed {
    #[serde(rename = "type")]
    pub media_type: String,
    pub mid: String,
    pub label: String,
    pub msid: Option<String>,
    pub age_ms: Option<u64>,
}

// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L3172-L3309
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct MountpointInfoRsp {
    pub info: MountpointInfo,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct MountpointInfo {
    pub id: JanusId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
    pub secret: Option<String>,
    pub pin: Option<String>,
    #[serde(default)]
    pub is_private: bool,
    pub enabled: bool,
    pub viewers: Option<u64>,
    /// <live|on demand>
    #[serde(rename = "type")]
    pub mountpoint_type: String,
    // TODO: add support for non RTP live mountpoints
    #[serde(default)]
    pub rtsp: bool,
    pub url: Option<String>,
    pub rtsp_user: Option<String>,
    pub rtsp_pwd: Option<String>,
    pub rtsp_quirk: Option<bool>,
    #[serde(default)]
    pub srtp: bool,
    #[serde(default)]
    pub collision: i32,
    #[serde(default)]
    pub threads: i32,
    pub host: Option<String>,
    pub media: Vec<RtpMediaInfo>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct RtpMediaInfo {
    pub mindex: u64,
    #[serde(rename = "type")]
    pub media_type: String,
    pub mid: String,
    pub label: String,
    pub msid: Option<String>,
    pub pt: Option<u8>, // 0-127
    pub codec: Option<String>,
    pub rtpmap: Option<String>,
    pub fmtp: Option<String>,
    #[serde(default)]
    pub videobufferkf: bool,
    #[serde(default)]
    pub videosimulcast: bool,
    #[serde(default)]
    pub videosvc: bool,
    #[serde(default)]
    pub skew_compensation: bool,
    pub port: Option<u16>,
    pub rtcpport: Option<u16>,
    pub port2: Option<u16>,
    pub port3: Option<u16>,
    // <text|binary>
    pub datatype: Option<String>,
    pub age_ms: Option<u64>,
    pub recording: Option<String>,
}
