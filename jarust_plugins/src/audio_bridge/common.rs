use serde::Deserialize;

use crate::common::Identifier;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
pub struct Participant {
    pub id: Identifier,
    pub display: Option<String>,
    pub setup: bool,
    pub muted: bool,
    pub suspended: Option<bool>,
    pub talking: Option<bool>,
    pub spatial_position: Option<u64>,
}
