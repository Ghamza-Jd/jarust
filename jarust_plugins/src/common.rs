use serde::Deserialize;
use serde::Serialize;

pub struct AttachPluginParams {
    /// Circular buffer capacity
    pub capacity: usize,
    // Request timeout
    pub timeout: std::time::Duration,
}

tryfrom_serde_value!(Identifier);

/// Rooms and Participants Identifier.
///
/// Identifier should be by default unsigned integer, unless configured otherwise in the plugin config.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Identifier {
    /// String Identifier
    String(String),
    /// Unsigned Integer Identifier
    Uint(u64),
}
