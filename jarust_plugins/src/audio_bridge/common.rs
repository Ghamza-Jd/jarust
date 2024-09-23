use crate::JanusId;
use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct AudioBridgeParticipant {
    pub id: JanusId,
    pub display: Option<String>,
    pub setup: bool,
    pub muted: bool,
    pub suspended: Option<bool>,
    pub talking: Option<bool>,
    pub spatial_position: Option<u64>,
}
