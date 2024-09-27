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
