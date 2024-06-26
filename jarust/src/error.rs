#[derive(Debug, thiserror::Error)]
pub enum JaError {
    /* Transformed Errors */
    #[error("Transport: {0}")]
    Transport(#[from] jarust_transport::error::JaTransportError),
    #[error("Failed to parse json: {0}")]
    JsonParsingFailure(#[from] serde_json::Error),
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    /* Custom Errors */
    #[error("Error while parsing an incomplete packet")]
    IncompletePacket,
    #[error("Transport is not opened")]
    TransportNotOpened,
    #[error("Invalid Janus request, reason: {reason}")]
    InvalidJanusRequest { reason: String },
    #[error("Can't send data in closed channel")]
    SendError,
    #[error("Received an unnexpected response")]
    UnexpectedResponse,
    #[error("Janus error {{ code: {code}, reason: {reason}}}")]
    JanusError { code: u16, reason: String },
    #[error("Request timeout")]
    RequestTimeout,
}
