use crate::japrotocol::JaResponse;
use crate::jarouter::JaRouter;
use crate::prelude::JaResult;
use crate::tmanager::TransactionManager;
use jarust_rt::JaTask;
use jarust_transport::trans::TransportProtocol;
use jarust_transport::trans::TransportSession;
use serde_json::Value;
use tokio::sync::mpsc;
use crate::demuxer::Demuxer;
use crate::error::JaError;

#[async_trait::async_trait]
pub(crate) trait NetworkConnection {
    async fn new(url: &str, namespace: &str, transport: impl TransportProtocol) -> JaResult<Self>
    where
        Self: Sized;

    async fn send(&mut self, message: Value) -> JaResult<String>;
    async fn add_subroute(&mut self, subroute: &str) -> mpsc::UnboundedReceiver<JaResponse>;
}

pub(crate) struct NwConn {
    namespace: String,
    tasks: Vec<JaTask>,
    router: JaRouter,
    transport: TransportSession,
    tmanager: TransactionManager,
}

#[async_trait::async_trait]
impl NetworkConnection for NwConn {
    async fn new(url: &str, namespace: &str, transport: impl TransportProtocol) -> JaResult<Self> {
        let (router, _) = JaRouter::new(namespace).await;
        let (transport, receiver) = TransportSession::connect(transport, url).await?;
        let tmanager = TransactionManager::new(32);

        let demux_task = jarust_rt::spawn({
            let router = router.clone();
            let tmanager = tmanager.clone();
            async move { Demuxer::demux_task(receiver, router, tmanager).await }
        });

        Ok(Self {
            namespace: namespace.into(),
            tasks: vec![demux_task],
            router,
            transport,
            tmanager: TransactionManager::new(32)
        })
    }

    async fn send(&mut self, message: Value) -> JaResult<String> {
        let Some(transaction) = message["transaction"].as_str() else {
            let err = JaError::InvalidJanusRequest {
                reason: "message transaction is missing".to_owned(),
            };
            tracing::error!("{err}");
            return Err(err);
        };

        let path = JaRouter::path_from_request(&message)
            .unwrap_or(self.namespace.clone());

        self.transport.send(message.to_string().as_bytes(), &path).await?;
        Ok(transaction.into())
    }

    async fn add_subroute(&mut self, subroute: &str) -> mpsc::UnboundedReceiver<JaResponse> {
        self.router.add_subroute(subroute).await
    }
}
