use std::time::Duration;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CreateConnectionParams {
    /// Keep alive interval in seconds
    pub ka_interval: u32,
    /// Buffer capacity
    pub capacity: usize,
    /// Request timeout
    pub timeout: Duration,
}
