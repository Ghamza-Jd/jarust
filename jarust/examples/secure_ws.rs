use jarust::jaconfig::ApiInterface;
use jarust::jaconfig::JaConfig;
use jarust::jaconnection::CreateConnectionParams;
use jarust::japlugin::Attach;
use jarust::japlugin::AttachHandleParams;
use jarust_transport::japrotocol::EstablishmentProtocol;
use jarust_transport::japrotocol::Jsep;
use jarust_transport::japrotocol::JsepType;
use jarust_transport::tgenerator::RandomTransactionGenerator;
use serde_json::json;
use std::path::Path;
use std::time::Duration;
use tokio::time;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let filename = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let env_filter = EnvFilter::from_default_env()
        .add_directive("jarust=trace".parse()?)
        .add_directive(format!("{filename}=trace").parse()?);
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let capacity = 32;
    let config = JaConfig::builder()
        .url("wss://janus.conf.meetecho.com/ws")
        .capacity(capacity)
        .build();
    let mut connection =
        jarust::connect(config, ApiInterface::WebSocket, RandomTransactionGenerator).await?;
    let timeout = Duration::from_secs(10);

    tracing::info!("server info: {:#?}", connection.server_info(timeout).await?);

    let session = connection
        .create(CreateConnectionParams {
            ka_interval: 10,
            timeout,
        })
        .await?;
    let (handle, mut event_receiver) = session
        .attach(AttachHandleParams {
            plugin_id: "janus.plugin.echotest".to_string(),
            timeout,
        })
        .await?;

    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(2));

        loop {
            handle
                .send_waiton_ack(
                    json!({
                        "video": true,
                        "audio": true,
                    }),
                    Duration::from_secs(10),
                )
                .await
                .unwrap();

            handle
                .fire_and_forget_with_establishment(
                    json!({
                        "video": true,
                        "audio": true,
                    }),
                    EstablishmentProtocol::JSEP(Jsep {
                        sdp: "".to_string(),
                        jsep_type: JsepType::Offer,
                    }),
                )
                .await
                .unwrap();

            interval.tick().await;
        }
    });

    while let Some(event) = event_receiver.recv().await {
        tracing::info!("response: {event:#?}");
    }

    Ok(())
}
