pub struct AttachPluginParams {
    /// Circular buffer capacity
    pub capacity: usize,
    // Request timeout
    pub timeout: std::time::Duration,
}
