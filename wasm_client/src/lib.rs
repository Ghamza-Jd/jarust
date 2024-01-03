use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust::japlugin::Attach;
use log::Level;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn connect(uri: &str) {
    console_log::init_with_level(Level::Trace).unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    runtime.block_on(async {
        let mut connection = jarust::connect(JaConfig::new(uri, None, TransportType::Wss, "janus"))
            .await
            .unwrap();
        let session = connection.create(10).await.unwrap();
        session.attach("janus.plugin.echotest").await.unwrap();
    });
}
