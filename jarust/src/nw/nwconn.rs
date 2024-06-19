use super::demuxer::Demuxer;
use super::jarouter::JaRouter;
use super::tmanager::TransactionManager;
use crate::error::JaError;
use crate::japrotocol::JaResponse;
use crate::prelude::JaResult;
use jarust_rt::JaTask;
use jarust_transport::trans::TransportProtocol;
use jarust_transport::trans::TransportSession;
use serde_json::Value;
use tokio::sync::mpsc;

#[async_trait::async_trait]
pub(crate) trait NetworkConnection {
    async fn new(
        url: &str,
        namespace: &str,
        transport: impl TransportProtocol,
    ) -> JaResult<(Self, mpsc::UnboundedReceiver<JaResponse>)>
    where
        Self: Sized;

    async fn send(&mut self, message: Value) -> JaResult<String>;
    async fn add_subroute(&mut self, subroute: &str) -> mpsc::UnboundedReceiver<JaResponse>;
}

#[derive(Debug)]
pub(crate) struct NwConn {
    namespace: String,
    tasks: Vec<JaTask>,
    router: JaRouter,
    transport: TransportSession,
    tmanager: TransactionManager,
}

#[async_trait::async_trait]
impl NetworkConnection for NwConn {
    async fn new(
        url: &str,
        namespace: &str,
        transport: impl TransportProtocol,
    ) -> JaResult<(Self, mpsc::UnboundedReceiver<JaResponse>)> {
        let (router, root_channel) = JaRouter::new(namespace).await;
        let (transport, receiver) = TransportSession::connect(transport, url).await?;
        let tmanager = TransactionManager::new(32);

        let demux_task = jarust_rt::spawn({
            let router = router.clone();
            let tmanager = tmanager.clone();
            async move { Demuxer::demux_task(receiver, router, tmanager).await }
        });

        Ok((
            Self {
                namespace: namespace.into(),
                tasks: vec![demux_task],
                router,
                transport,
                tmanager,
            },
            root_channel,
        ))
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    async fn send(&mut self, message: Value) -> JaResult<String> {
        let Some(transaction) = message["transaction"].as_str() else {
            let err = JaError::InvalidJanusRequest {
                reason: "message transaction is missing".to_owned(),
            };
            tracing::error!("{err}");
            return Err(err);
        };

        let path = JaRouter::path_from_request(&message).unwrap_or(self.namespace.clone());

        self.tmanager.create_transaction(transaction, &path).await;
        self.transport
            .send(message.to_string().as_bytes(), &path)
            .await?;
        tracing::debug!("{message:#?}");
        Ok(transaction.into())
    }

    async fn add_subroute(&mut self, subroute: &str) -> mpsc::UnboundedReceiver<JaResponse> {
        self.router.add_subroute(subroute).await
    }
}

impl Drop for NwConn {
    fn drop(&mut self) {
        self.tasks.iter().for_each(|task| {
            task.cancel();
        });
    }
}
