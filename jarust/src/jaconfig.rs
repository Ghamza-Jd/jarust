#[derive(Debug)]
pub struct JaConfig {
    pub uri: String,
    pub apisecret: Option<String>,
    pub transport_type: TransportType,
    pub root_namespace: String,
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
