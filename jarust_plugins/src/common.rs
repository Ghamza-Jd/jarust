use serde::{Deserialize, Serialize};

use crate::Identifier::Uint;

pub struct AttachPluginParams {
    /// Circular buffer capacity
    pub capacity: usize,
    // Request timeout
    pub timeout: std::time::Duration,
}

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
        Uint(0)
    }
}
