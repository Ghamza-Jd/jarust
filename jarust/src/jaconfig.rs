#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct JaConfig {
    pub url: String,
    pub apisecret: Option<String>,
    pub server_root: String,
    pub capacity: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum JanusAPI {
    WebSocket,
    Restful,
}
