use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::japlugin::Attach;
use jarust::japrotocol::EstablishmentProtocol;
use jarust::japrotocol::Jsep;
use jarust::japrotocol::JsepType;
use serde_json::json;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust=trace".parse()?))
        .init();

    let mut connection = jarust::connect(
        JaConfig::new("wss://janus.conf.meetecho.com/ws", None, "janus"),
        TransportType::Ws,
    )
    .await?;
    let session = connection.create(10).await?;
    let (handle, mut event_receiver) = session.attach("janus.plugin.echotest").await?;

    handle
        .fire_and_forget(json!({
            "video": true,
            "audio": true,
        }))
        .await?;

    handle
        .send_waiton_ack_with_establishment(
            json!({
                "video": true,
                "audio": true,
            }),
            EstablishmentProtocol::JSEP(Jsep {
                sdp: "".to_string(),
                jsep_type: JsepType::Offer,
            }),
            Duration::from_secs(10),
        )
        .await?;

    while let Some(event) = event_receiver.recv().await {
        tracing::info!("response: {event:#?}");
    }

    Ok(())
}
