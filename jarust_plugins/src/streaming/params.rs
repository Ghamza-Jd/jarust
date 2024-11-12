use crate::JanusId;
use serde::Serialize;

// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L3311-L4175
// TODO: only RTP type is supported
make_dto!(
    StreamingCreateParams,
    required {
        #[serde(rename = "type")]
        mountpoint_type: StreamingMountpointType
    },
    optional {
        admin_key: String,
        id: JanusId,
        name: String,
        description: String,
        metadata: String,
        is_private: bool,
        secret: String,
        /// pin required for viewers to access mountpoint
        pin: String,
        /// whether the mountpoint should be saved to the configuration file or not, default=false
        permanent: bool,
        // RTP only
        media: Vec<StreamingRtpMedia>
    }
);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamingMountpointType {
    RTP,
    LIVE,
    ONDEMAND,
    RTSP,
}

// https://github.com/meetecho/janus-gateway/blob/v1.2.4/src/plugins/janus_streaming.c#L1100
make_dto!(
    StreamingRtpMedia,
    required {
        #[serde(rename = "type")]
        media_type: StreamingRtpMediaType,
        /// Unique mid to assign to this stream in negociated PeerConnections
        mid: String,
        port: u16
    },
    optional {
        label: String,
        msid: String,
        mcast: String,
        iface: String,
        rtcpport: u16,
        pt: u8, // payload type is restricted to 0-127
        codec: String,
        fmtp: String,
        skew: bool // missing video only and data only parameters ?
    }
);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamingRtpMediaType {
    AUDIO,
    VIDEO,
    DATA,
}

make_dto!(
    StreamingDestroyParams,
    required { id: JanusId },
    optional {
        /// mountpoint secret, mandatory if configured
        secret: String,
        /// whether the mountpoint should be also removed from the config file, default=false
        permanent: bool
    }
);
