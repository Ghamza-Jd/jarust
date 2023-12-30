pub(crate) const CHANNEL_BUFFER_SIZE: usize = 32;

#[derive(Debug)]
pub struct JaConfig {
    pub(crate) uri: String,
    pub(crate) apisecret: Option<String>,
    pub(crate) transport_type: TransportType,
    pub(crate) root_namespace: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportType {
    Wss,
}

impl JaConfig {
    pub fn new(
        uri: &str,
        apisecret: Option<String>,
        transport_type: TransportType,
        root_namespace: &str,
    ) -> Self {
        Self {
            uri: uri.into(),
            apisecret,
            transport_type,
            root_namespace: root_namespace.into(),
        }
    }
}
