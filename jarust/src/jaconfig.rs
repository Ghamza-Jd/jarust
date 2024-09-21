#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct JaConfig {
    pub(crate) url: String,
    pub(crate) apisecret: Option<String>,
    pub(crate) server_root: String,
    pub(crate) capacity: usize,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ApiInterface {
    WebSocket,
    Restful,
}

impl JaConfig {
    pub fn builder() -> JaConfigBuilder<NoUrlTypeState, NoCapacityTypeState> {
        JaConfigBuilder {
            url: NoUrlTypeState,
            apisecret: None,
            server_root: None,
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
    pub(crate) server_root: Option<String>,
    pub(crate) capacity: C,
}

impl<C> JaConfigBuilder<NoUrlTypeState, C> {
    /// Set the URL of the Janus server.
    pub fn url(self, url: &str) -> JaConfigBuilder<WithUrlTypeState, C> {
        let Self {
            apisecret,
            server_root,
            capacity,
            ..
        } = self;
        JaConfigBuilder {
            apisecret,
            server_root,
            capacity,
            url: WithUrlTypeState(url.into()),
        }
    }
}

impl<U> JaConfigBuilder<U, NoCapacityTypeState> {
    /// Set the capacity for the Janus client ring buffer
    ///
    /// Mandatory for WebSocket and does nothing for Restful
    pub fn capacity(self, cap: usize) -> JaConfigBuilder<U, WithCapacityTypeState> {
        let Self {
            apisecret,
            server_root,
            url,
            ..
        } = self;
        JaConfigBuilder {
            apisecret,
            server_root,
            url,
            capacity: WithCapacityTypeState(cap),
        }
    }
}

impl<U, C> JaConfigBuilder<U, C> {
    /// Set the API secret for the Janus server.
    pub fn apisecret(self, apisecret: &str) -> Self {
        let Self {
            server_root,
            url,
            capacity,
            ..
        } = self;
        JaConfigBuilder {
            apisecret: Some(apisecret.into()),
            server_root,
            url,
            capacity,
        }
    }

    /// Set the server root for the Janus server (default "janus")
    ///
    /// It's overridable for WebSocket (it's not critical for ws)
    ///
    /// It's mandatory for Restful as it should match the server_root in the janus config file or it will result in 404s
    pub fn server_root(self, server_root: &str) -> Self {
        let Self {
            apisecret,
            url,
            capacity,
            ..
        } = self;
        JaConfigBuilder {
            server_root: Some(server_root.into()),
            apisecret,
            url,
            capacity,
        }
    }
}

impl JaConfigBuilder<WithUrlTypeState, WithCapacityTypeState> {
    pub fn build(self) -> JaConfig {
        let Self {
            server_root,
            apisecret,
            url,
            capacity,
        } = self;
        let server_root = server_root.unwrap_or(String::from("janus"));
        JaConfig {
            server_root,
            apisecret,
            url: url.0,
            capacity: capacity.0,
        }
    }
}
