use super::trans::Transport;
use crate::jaconfig::CHANNEL_BUFFER_SIZE;
use crate::jatask;
use crate::jatask::AbortHandle;
use crate::prelude::*;
use async_trait::async_trait;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use futures_util::StreamExt;
use rustls::RootCertStore;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async_tls_with_config;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::Connector;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;

type WebSocketSender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

pub struct WebsocketTransport {
    sender: Option<WebSocketSender>,
    abort_handle: Option<AbortHandle>,
}

#[async_trait]
impl Transport for WebsocketTransport {
    fn create_transport() -> Self {
        Self {
            sender: None,
            abort_handle: None,
        }
    }

    async fn connect(&mut self, uri: &str) -> JaResult<mpsc::Receiver<String>> {
        let mut request = uri.into_client_request()?;
        let headers = request.headers_mut();
        headers.insert("Sec-Websocket-Protocol", "janus-protocol".parse()?);

        let connector = Connector::Rustls(WebsocketTransport::make_tls_client_config()?);
        let (stream, _) =
            connect_async_tls_with_config(request, None, true, Some(connector)).await?;

        let (sender, mut receiver) = stream.split();
        let (tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);

        let abort_handle = jatask::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                if let Message::Text(text) = message {
                    let _ = tx.send(text).await;
                }
            }
        });

        self.sender = Some(sender);
        self.abort_handle = Some(abort_handle);
        Ok(rx)
    }

    async fn send(&mut self, data: &[u8]) -> JaResult<()> {
        let item = Message::Binary(data.to_vec());
        if let Some(sender) = &mut self.sender {
            sender.send(item).await?;
        } else {
            tracing::error!("Transport not opened!");
            return Err(JaError::TransportNotOpened);
        }
        Ok(())
    }
}

impl WebsocketTransport {
    fn make_tls_client_config() -> JaResult<Arc<rustls::ClientConfig>> {
        let mut root_store = RootCertStore::empty();
        let platform_certs = rustls_native_certs::load_native_certs()?;
        root_store.add_parsable_certificates(platform_certs);
        let client_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        Ok(Arc::new(client_config))
    }
}

impl Drop for WebsocketTransport {
    #[tracing::instrument(parent = None, level = tracing::Level::TRACE, skip(self))]
    fn drop(&mut self) {
        if let Some(join_handle) = self.abort_handle.take() {
            tracing::debug!("Dropping wss transport");
            join_handle.abort();
        }
    }
}

impl Debug for WebsocketTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Websocket").finish()
    }
}
