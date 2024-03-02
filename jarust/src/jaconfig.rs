pub const CHANNEL_BUFFER_SIZE: usize = 32;

#[derive(Debug)]
pub struct JaConfig {
    pub(crate) uri: String,
    pub(crate) apisecret: Option<String>,
    pub(crate) namespace: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportType {
    Ws,
}

impl JaConfig {
    pub fn builder() -> JaConfigBuilder<NoUrlTypeState> {
        JaConfigBuilder {
            url: NoUrlTypeState,
            apisecret: None,
            namespace: None,
        }
    }
}

pub struct NoUrlTypeState;
pub struct WithUrlTypeState(pub(crate) String);

pub struct JaConfigBuilder<U> {
    pub(crate) url: U,
    pub(crate) apisecret: Option<String>,
    pub(crate) namespace: Option<String>,
}

impl JaConfigBuilder<NoUrlTypeState> {
    pub fn url(self, url: &str) -> JaConfigBuilder<WithUrlTypeState> {
        let Self {
            apisecret,
            namespace,
            ..
        } = self;
        JaConfigBuilder {
            apisecret,
            namespace,
            url: WithUrlTypeState(url.into()),
        }
    }
}

impl<T> JaConfigBuilder<T> {
    pub fn apisecret(self, apisecret: &str) -> Self {
        let Self { namespace, url, .. } = self;
        JaConfigBuilder {
            apisecret: Some(apisecret.into()),
            namespace,
            url,
        }
    }

    pub fn namespace(self, namespace: &str) -> Self {
        let Self { apisecret, url, .. } = self;
        JaConfigBuilder {
            namespace: Some(namespace.into()),
            apisecret,
            url,
        }
    }
}

impl JaConfigBuilder<WithUrlTypeState> {
    pub fn build(self) -> JaConfig {
        let Self {
            namespace,
            apisecret,
            url,
        } = self;
        let namespace = namespace.unwrap_or(String::from("jarust"));
        JaConfig {
            namespace: namespace,
            apisecret: apisecret,
            uri: url.0,
        }
    }
}
