#[derive(thiserror::Error, Debug)]
pub enum JaTransportError {
    /* Transformed Errors */
    #[cfg(not(target_family = "wasm"))]
    #[error("Websocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[cfg(not(target_family = "wasm"))]
    #[error("InvalidHeaderValue: {0}")]
    InvalidHeaderValue(#[from] tokio_tungstenite::tungstenite::http::header::InvalidHeaderValue),

    /* Custom Errors */
    #[error("Transport is not opened")]
    TransportNotOpened,
}
