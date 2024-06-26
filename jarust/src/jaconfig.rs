#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct JaConfig {
    pub(crate) url: String,
    pub(crate) apisecret: Option<String>,
    pub(crate) namespace: String,
    pub(crate) capacity: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TransportType {
    Ws,
}

impl JaConfig {
    pub fn builder() -> JaConfigBuilder<NoUrlTypeState, NoCapacityTypeState> {
        JaConfigBuilder {
            url: NoUrlTypeState,
            apisecret: None,
            namespace: None,
            capacity: NoCapacityTypeState,
        }
    }
}

pub struct NoUrlTypeState;
pub struct WithUrlTypeState(pub(crate) String);

pub struct NoCapacityTypeState;
pub struct WithCapacityTypeState(pub(crate) usize);

pub struct JaConfigBuilder<U, C> {
    pub(crate) url: U,
    pub(crate) apisecret: Option<String>,
    pub(crate) namespace: Option<String>,
    pub(crate) capacity: C,
}

impl<C> JaConfigBuilder<NoUrlTypeState, C> {
    pub fn url(self, url: &str) -> JaConfigBuilder<WithUrlTypeState, C> {
        let Self {
            apisecret,
            namespace,
            capacity,
            ..
        } = self;
        JaConfigBuilder {
            apisecret,
            namespace,
            capacity,
            url: WithUrlTypeState(url.into()),
        }
    }
}

impl<U> JaConfigBuilder<U, NoCapacityTypeState> {
    pub fn capacity(self, cap: usize) -> JaConfigBuilder<U, WithCapacityTypeState> {
        let Self {
            apisecret,
            namespace,
            url,
            ..
        } = self;
        JaConfigBuilder {
            apisecret,
            namespace,
            url,
            capacity: WithCapacityTypeState(cap),
        }
    }
}

impl<U, C> JaConfigBuilder<U, C> {
    pub fn apisecret(self, apisecret: &str) -> Self {
        let Self {
            namespace,
            url,
            capacity,
            ..
        } = self;
        JaConfigBuilder {
            apisecret: Some(apisecret.into()),
            namespace,
            url,
            capacity,
        }
    }

    pub fn namespace(self, namespace: &str) -> Self {
        let Self {
            apisecret,
            url,
            capacity,
            ..
        } = self;
        JaConfigBuilder {
            namespace: Some(namespace.into()),
            apisecret,
            url,
            capacity,
        }
    }
}

impl JaConfigBuilder<WithUrlTypeState, WithCapacityTypeState> {
    pub fn build(self) -> JaConfig {
        let Self {
            namespace,
            apisecret,
            url,
            capacity,
        } = self;
        let namespace = namespace.unwrap_or(String::from("jarust"));
        JaConfig {
            namespace,
            apisecret,
            url: url.0,
            capacity: capacity.0,
        }
    }
}
