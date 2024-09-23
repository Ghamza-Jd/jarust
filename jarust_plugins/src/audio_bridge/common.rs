use serde::Deserialize;

use crate::JanusId;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct Participant {
    pub id: JanusId,
    pub display: Option<String>,
    pub setup: bool,
    pub muted: bool,
    pub suspended: Option<bool>,
    pub talking: Option<bool>,
    pub spatial_position: Option<u64>,
}
