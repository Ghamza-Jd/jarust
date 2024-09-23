use serde::Deserialize;
use serde::Serialize;

impl_tryfrom_serde_value!(JanusId);

/// Rooms and Participants Identifier.
///
/// Identifier should be by default unsigned integer, unless configured otherwise in the plugin config.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JanusId {
    /// String Identifier
    String(String),
    /// Unsigned Integer Identifier
    Uint(u64),
}

impl Default for JanusId {
    fn default() -> Self {
        Self::Uint(0)
    }
}
