use std::time::Duration;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CreateConnectionParams {
    /// Keep alive interval in seconds
    pub ka_interval: u32,
    /// Circular buffer capacity
    pub capacity: usize,
    /// Request timeout
    pub timeout: Duration,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AttachHandleParams {
    /// Janus plugin identifier
    pub plugin_id: String,
    /// Request timeout
    pub timeout: Duration,
}
