use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerInfoRsp {
    pub name: String,
    pub version: u64,
    #[serde(rename = "version_string")]
    pub version_string: String,
    pub author: String,
    pub commit_hash: String,
    pub compile_time: String,
    pub log_to_stdout: bool,
    pub log_to_file: bool,
    #[serde(rename = "data_channels")]
    pub data_channels: bool,
    pub accepting_new_sessions: bool,
    pub session_timeout: u64,
    pub reclaim_session_timeout: u64,
    pub candidates_timeout: u64,
    pub server_name: String,
    pub local_ip: String,
    pub ipv6: bool,
    pub ice_lite: bool,
    pub ice_tcp: bool,
    pub ice_nomination: String, /* could be enum when we know the variants */
    pub ice_keepalive_conncheck: bool,
    pub full_trickle: bool,
    pub mdns_enabled: bool,
    pub min_nack_queue: u64,
    pub twcc_period: u64,
    pub dtls_mtu: u64,
    pub static_event_loops: u64,
    #[serde(rename = "api_secret")]
    pub api_secret: bool,
    #[serde(rename = "auth_token")]
    pub auth_token: bool,
    #[serde(rename = "event_handlers")]
    pub event_handlers: bool,
    #[serde(rename = "opaqueid_in_api")]
    pub opaqueid_in_api: bool,
    pub dependencies: HashMap<String, String>,
    pub transports: HashMap<String, MetaData>,
    pub plugins: HashMap<String, MetaData>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct MetaData {
    name: String,
    author: String,
    description: String,
    version_string: String,
    version: u64,
}
