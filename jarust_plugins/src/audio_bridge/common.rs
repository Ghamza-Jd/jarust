use serde::Deserialize;
use serde::Serialize;

/// Rooms and Participants Identifier.
///
/// Identifier should be by default unsigned integer, unless configured otherwise in the audiobridge config.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Identifier {
    /// String Identifier
    String(String),
    /// Unsigned Integer Identifier
    Uint(u64),
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Participant {
    pub id: Identifier,
    pub display: Option<String>,
    pub setup: bool,
    pub muted: bool,
    pub suspended: Option<bool>,
    pub talking: Option<bool>,
    pub spatial_position: Option<u64>,
}
