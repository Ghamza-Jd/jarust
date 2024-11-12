use serde::Deserialize;
use serde::Serialize;

impl_tryfrom_serde_value!(JanusId);

/// Mountpoints, Rooms and Participants Identifier.
///
/// Identifier should be by default unsigned integer, unless configured otherwise in the plugin config.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JanusId {
    /// String Identifier
    String(String),
    /// Unsigned Integer Identifier
    Uint(U63),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct U63(u64);

impl U63 {
    pub const MAX: u64 = (1 << 63) - 1;

    pub fn new(value: u64) -> Self {
        Self::new_wrap(value)
    }

    pub fn new_wrap(value: u64) -> Self {
        Self(value & U63::MAX)
    }

    pub fn new_saturating(value: u64) -> Self {
        if value > U63::MAX {
            Self(U63::MAX)
        } else {
            Self(value)
        }
    }
}
