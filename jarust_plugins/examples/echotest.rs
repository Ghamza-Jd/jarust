use jarust::jaconfig::JaConfig;
use jarust::jaconfig::JanusAPI;
use jarust::jaconnection::CreateConnectionParams;
use jarust_interface::tgenerator::RandomTransactionGenerator;
use jarust_plugins::echo_test::events::EchoTestEvent;
use jarust_plugins::echo_test::events::PluginEvent;
use jarust_plugins::echo_test::jahandle_ext::EchoTest;
use jarust_plugins::echo_test::msg_options::EchoTestStartOptions;
use std::path::Path;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let filename = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust=trace".parse()?)
        .add_directive("jarust_plugins=trace".parse()?)
        .add_directive("jarust_interface=trace".parse()?)
        .add_directive("jarust_rt=trace".parse()?)
        .add_directive(format!("{filename}=trace").parse()?);
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let capacity = 32;
    let config = JaConfig::builder()
        .url("wss://janus.conf.meetecho.com/ws")
        .capacity(capacity)
        .build();
    let mut connection =
        jarust::connect(config, JanusAPI::WebSocket, RandomTransactionGenerator).await?;
    let timeout = Duration::from_secs(10);
    let session = connection
        .create_session(CreateConnectionParams {
            ka_interval: 10,
            timeout,
        })
        .await?;
    let (handle, mut event_receiver) = session.attach_echo_test(timeout).await?;

    handle
        .start(EchoTestStartOptions {
            audio: true,
            video: true,
            ..Default::default()
        })
        .await?;

    while let Some(event) = event_receiver.recv().await {
        match event {
            PluginEvent::EchoTestEvent(EchoTestEvent::Result { result, .. }) => {
                tracing::info!("result: {result}");
            }
            PluginEvent::EchoTestEvent(EchoTestEvent::ResultWithEstablishment {
                establishment_protocol,
                ..
            }) => {
                tracing::info!("establishment_protocol: {establishment_protocol:#?}");
            }
            PluginEvent::EchoTestEvent(EchoTestEvent::Error { error_code, error }) => {
                tracing::warn!("error: {{ error_code: {error_code}, error: {error} }}");
            }
            PluginEvent::GenericEvent(event) => {
                tracing::debug!("generic event: {event:#?}");
            }
        }
    }

    Ok(())
}
