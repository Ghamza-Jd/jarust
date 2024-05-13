#[cfg(all(feature = "use-rustls", feature = "use-native-tls"))]
compile_error!("Feature \"rustls\" and feature \"native-tls\" cannot be enabled at the same time");

#[cfg(not(any(feature = "use-rustls", feature = "use-native-tls")))]
compile_error!("Either feature \"rustls\" or \"native-tls\" must be enabled for this crate");

use super::trans::TransportProtocol;
use crate::error::JaTransportError;
use crate::prelude::*;
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use futures_util::StreamExt;
use jarust_rt::JaTask;
use std::fmt::Debug;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::handshake::client::Request;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;

#[cfg(feature = "use-rustls")]
use rustls::RootCertStore;
#[cfg(feature = "use-rustls")]
use std::sync::Arc;
#[cfg(feature = "use-rustls")]
use tokio_tungstenite::connect_async_tls_with_config;
#[cfg(feature = "use-native-tls")]
use tokio_tungstenite::connect_async_with_config;
#[cfg(feature = "use-rustls")]
use tokio_tungstenite::Connector;

type WebSocketSender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

pub struct WebsocketTransport {
    sender: Option<WebSocketSender>,
    task: Option<JaTask>,
}

#[async_trait]
impl TransportProtocol for WebsocketTransport {
    fn create_transport() -> Self {
        Self {
            sender: None,
            task: None,
        }
    }

    async fn connect(&mut self, uri: &str) -> JaTransportResult<mpsc::UnboundedReceiver<Bytes>> {
        let mut request = uri.into_client_request()?;
        let headers = request.headers_mut();
        headers.insert("Sec-Websocket-Protocol", "janus-protocol".parse()?);
        let stream = Self::connect_async(request).await?;

        let (sender, mut receiver) = stream.split();
        let (tx, rx) = mpsc::unbounded_channel();

        let task = jarust_rt::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                if let Message::Text(text) = message {
                    let _ = tx.send(text.into());
                }
            }
        });

        self.sender = Some(sender);
        self.task = Some(task);
        Ok(rx)
    }

    async fn send(&mut self, data: &[u8]) -> JaTransportResult<()> {
        let item = Message::Binary(data.to_vec());
        if let Some(sender) = &mut self.sender {
            sender.send(item).await?;
        } else {
            tracing::error!("Transport not opened!");
            return Err(JaTransportError::TransportNotOpened);
        }
        Ok(())
    }
}

impl WebsocketTransport {
    #[cfg(feature = "use-rustls")]
    fn make_tls_client_config() -> JaResult<Arc<rustls::ClientConfig>> {
        let mut root_store = RootCertStore::empty();
        let platform_certs = rustls_native_certs::load_native_certs()?;
        root_store.add_parsable_certificates(platform_certs);
        let client_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        Ok(Arc::new(client_config))
    }

    async fn connect_async(
        request: Request,
    ) -> JaTransportResult<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        #[cfg(feature = "use-rustls")]
        {
            let connector = Connector::Rustls(WebsocketTransport::make_tls_client_config()?);
            let (stream, ..) =
                connect_async_tls_with_config(request, None, true, Some(connector)).await?;
            Ok(stream)
        }

        #[cfg(feature = "use-native-tls")]
        {
            let (stream, ..) = connect_async_with_config(request, None, false).await?;
            Ok(stream)
        }
    }
}

impl Drop for WebsocketTransport {
    #[tracing::instrument(parent = None, level = tracing::Level::TRACE, skip(self))]
    fn drop(&mut self) {
        if let Some(join_handle) = self.task.take() {
            tracing::debug!("Dropping wss transport");
            join_handle.cancel();
        }
    }
}

impl Debug for WebsocketTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Websocket").finish()
    }
}
