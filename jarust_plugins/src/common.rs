use serde::Deserialize;
use serde::Serialize;

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

impl Default for Identifier {
    fn default() -> Self {
        Self::Uint(0)
    }
}
