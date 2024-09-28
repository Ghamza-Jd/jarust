#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    JarustInterface(#[from] jarust_interface::Error),
    #[error("Failed to parse json: {0}")]
    JsonParsingFailure(#[from] serde_json::Error),
    #[error("Audio bridge {{ error_code: {error_code}, error: {error} }}")]
    AudioBridge { error_code: u16, error: String },
}
