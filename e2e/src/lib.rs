use tracing_subscriber::EnvFilter;

/// For debuggin e2e tests
pub fn init_tracing_subscriber() {
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust_core=trace".parse().unwrap())
        .add_directive("jarust_plugins=trace".parse().unwrap())
        .add_directive("jarust_interface=trace".parse().unwrap())
        .add_directive("jarust_rt=trace".parse().unwrap());
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}
