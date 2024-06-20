use super::jarouter::JaRouter;
use super::transaction_manager::TransactionManager;
use crate::japrotocol::ResponseType;
use crate::prelude::*;
use bytes::Bytes;
use tokio::sync::mpsc;

pub(crate) struct Demuxer;

impl Demuxer {
    /// Async task to handle demultiplexing of the inbound stream
    #[tracing::instrument(name = "incoming_event", level = tracing::Level::TRACE, skip_all)]
    pub(crate) async fn demux_task(
        inbound_stream: mpsc::UnboundedReceiver<Bytes>,
        router: JaRouter,
        transaction_manager: TransactionManager,
    ) -> JaResult<()> {
        let mut stream = inbound_stream;
        while let Some(next) = stream.recv().await {
            let Ok(incoming_event) = std::str::from_utf8(&next) else {
                tracing::error!("Incomplete packet received");
                continue;
            };

            tracing::debug!("Received {incoming_event}");

            // Parse the incoming message
            let message = match serde_json::from_str::<JaResponse>(incoming_event) {
                Ok(response) => match &response.janus {
                    ResponseType::Error { error } => {
                        tracing::error!("{error:#?}");
                        response
                    }
                    _ => response,
                },
                Err(what) => {
                    tracing::error!("Error parsing response: {what}");
                    continue;
                }
            };

            // Try send the message to the proper route
            if let Err(what) = Self::demux_message(message, &router, &transaction_manager).await {
                tracing::error!("Error demuxing message: {what}");
            }
        }
        Ok(())
    }

    /// Route the message to the proper channel
    async fn demux_message(
        message: JaResponse,
        router: &JaRouter,
        transaction_manager: &TransactionManager,
    ) -> JaResult<()> {
        // Check if we have a pending transaction and demux to the proper route
        if let Some(transaction) = message.transaction.clone() {
            if let Some(path) = transaction_manager.get(&transaction).await {
                if path == router.root_path() {
                    router.pub_root(message).await?;
                } else {
                    router.pub_subroute(&path, message).await?;
                }
                return Ok(());
            }
        }

        // Try get the route from the response
        if let Some(path) = JaRouter::path_from_response(message.clone()) {
            router.pub_subroute(&path, message).await?;
            return Ok(());
        }

        // Fallback to publishing on the root route
        router.pub_root(message).await?;
        Ok(())
    }
}
