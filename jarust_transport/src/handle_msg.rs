use super::japrotocol::EstablishmentProtocol;
use serde_json::Value;
use std::time::Duration;

pub struct HandleMessage {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
}

pub struct HandleMessageWithTimeout {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
    pub timeout: Duration,
}

pub struct HandleMessageWithEstablishment {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
    pub protocol: EstablishmentProtocol,
}

pub struct HandleMessageWithEstablishmentAndTimeout {
    pub session_id: u64,
    pub handle_id: u64,
    pub body: Value,
    pub timeout: Duration,
    pub protocol: EstablishmentProtocol,
}
