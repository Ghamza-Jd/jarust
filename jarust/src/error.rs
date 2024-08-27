#[derive(Debug, thiserror::Error)]
pub enum JaError {
    /* Transformed Errors */
    #[error("Transport: {0}")]
    JanusTransport(#[from] jarust_transport::error::JaTransportError),
    #[error("Failed to parse json: {0}")]
    JsonParsingFailure(#[from] serde_json::Error),
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    /* Custom Errors */
    #[error("Error while parsing an incomplete packet")]
    IncompletePacket,
    #[error("Invalid Janus request, reason: {reason}")]
    InvalidJanusRequest { reason: String },
}
