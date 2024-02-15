use crate::jarouter::JaRouter;
use crate::prelude::*;
use crate::tmanager::TransactionManager;
use tokio::sync::mpsc;

pub(crate) struct Demuxer;

impl Demuxer {
    /// Async task to handle demultiplexing of the inbound stream
    #[tracing::instrument(name = "incoming_event", level = tracing::Level::TRACE, skip_all)]
    pub(crate) async fn demux_task(
        inbound_stream: mpsc::Receiver<String>,
        router: JaRouter,
        transaction_manager: TransactionManager,
    ) -> JaResult<()> {
        let mut stream = inbound_stream;
        while let Some(next) = stream.recv().await {
            tracing::debug!("Received {next}");
            let message = serde_json::from_str::<JaResponse>(&next)?;

            // Check if we have a pending transaction and demux to the proper route
            if let Some(pending) = message
                .transaction
                .clone()
                .and_then(|x| transaction_manager.get(&x))
            {
                if pending.path == router.root_path() {
                    router.pub_root(message).await?;
                } else {
                    router.pub_subroute(&pending.path, message).await?;
                }
                transaction_manager.success_close(&pending.id);
                continue;
            }

            // Try get the route from the response
            if let Some(path) = JaRouter::path_from_response(message.clone()) {
                router.pub_subroute(&path, message).await?;
                continue;
            }

            // Fallback to publishing on the root route
            router.pub_root(message).await?;
        }
        Ok(())
    }
}
