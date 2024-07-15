use super::demuxer::Demuxer;
use super::router::Router;
use super::transaction_gen::TransactionGenerator;
use super::transaction_manager::TransactionManager;
use crate::japrotocol::JaResponse;
use crate::prelude::JaResult;
use crate::GenerateTransaction;
use jarust_rt::JaTask;
use jarust_transport::trans::TransportProtocol;
use jarust_transport::trans::TransportSession;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[derive(Debug)]
struct Shared {
    task: JaTask,
    namespace: String,
    apisecret: Option<String>,
    transaction_generator: TransactionGenerator,
}

#[derive(Debug)]
struct Exclusive {
    router: Router,
    transport: TransportSession,
    transaction_manager: TransactionManager,
}

#[derive(Debug)]
struct InnerJaTransport {
    shared: Shared,
    exclusive: Mutex<Exclusive>,
}

#[derive(Debug, Clone)]
pub struct JaTransport {
    inner: Arc<InnerJaTransport>,
}

pub struct ConnectionParams<'a> {
    pub url: &'a str,
    pub capacity: usize,
    pub apisecret: Option<String>,
    pub namespace: &'a str,
}

impl JaTransport {
    pub async fn new(
        conn_params: ConnectionParams<'_>,
        transport: impl TransportProtocol,
        transaction_generator: impl GenerateTransaction,
    ) -> JaResult<(Self, mpsc::UnboundedReceiver<JaResponse>)> {
        let (router, root_channel) = Router::new(conn_params.namespace).await;
        let (transport, receiver) = TransportSession::connect(transport, conn_params.url).await?;
        let transaction_manager = TransactionManager::new(conn_params.capacity);
        let transaction_generator = TransactionGenerator::new(transaction_generator);

        let demux_task = jarust_rt::spawn({
            let router = router.clone();
            let transaction_manager = transaction_manager.clone();
            async move { Demuxer::demux_task(receiver, router, transaction_manager).await }
        });

        let shared = Shared {
            task: demux_task,
            namespace: conn_params.namespace.into(),
            apisecret: conn_params.apisecret,
            transaction_generator,
        };
        let exclusive = Exclusive {
            router,
            transport,
            transaction_manager,
        };
        let inner = InnerJaTransport {
            shared,
            exclusive: Mutex::new(exclusive),
        };
        let this = Self {
            inner: Arc::new(inner),
        };
        Ok((this, root_channel))
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip_all)]
    pub async fn send(&self, message: Value) -> JaResult<String> {
        let (message, transaction) = self.decorate_request(message);

        let path =
            Router::path_from_request(&message).unwrap_or(self.inner.shared.namespace.clone());

        let mut guard = self.inner.exclusive.lock().await;
        guard.transaction_manager.insert(&transaction, &path).await;
        guard
            .transport
            .send(message.to_string().as_bytes(), &path)
            .await?;
        tracing::debug!("{message:#?}");
        Ok(transaction)
    }

    pub async fn add_session_subroute(
        &self,
        session_id: u64,
    ) -> mpsc::UnboundedReceiver<JaResponse> {
        self.inner
            .exclusive
            .lock()
            .await
            .router
            .add_subroute(&format!("{session_id}"))
            .await
    }

    pub async fn add_handle_subroute(
        &self,
        session_id: u64,
        handle_id: u64,
    ) -> mpsc::UnboundedReceiver<JaResponse> {
        self.inner
            .exclusive
            .lock()
            .await
            .router
            .add_subroute(&format!("{session_id}/{handle_id}"))
            .await
    }

    fn decorate_request(&self, mut request: Value) -> (Value, String) {
        let transaction = self
            .inner
            .shared
            .transaction_generator
            .generate_transaction();
        if let Some(apisecret) = self.inner.shared.apisecret.clone() {
            request["apisecret"] = apisecret.into();
        };
        request["transaction"] = transaction.clone().into();
        (request, transaction)
    }
}

impl Drop for InnerJaTransport {
    fn drop(&mut self) {
        self.shared.task.cancel();
    }
}
