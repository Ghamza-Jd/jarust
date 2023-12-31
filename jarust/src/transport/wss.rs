use super::trans::Transport;
use crate::jaconfig::CHANNEL_BUFFER_SIZE;
use crate::prelude::*;
use async_trait::async_trait;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task::AbortHandle;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;

type WebSocketSender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

pub struct WebsocketTransport {
    sender: Option<WebSocketSender>,
    abort_handle: Option<AbortHandle>,
}

#[async_trait]
impl Transport for WebsocketTransport {
    fn new() -> Self {
        Self {
            sender: None,
            abort_handle: None,
        }
    }

    async fn connect(&mut self, uri: &str) -> JaResult<mpsc::Receiver<String>> {
        let mut request = uri.into_client_request()?;
        let headers = request.headers_mut();
        headers.insert("Sec-Websocket-Protocol", "janus-protocol".parse()?);
        let (stream, _) = connect_async(request).await?;
        let (sender, mut receiver) = stream.split();
        let (tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);

        let forward_join_handle = tokio::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                if let Message::Text(text) = message {
                    let _ = tx.send(text).await;
                }
            }
        });

        self.sender = Some(sender);
        self.abort_handle = Some(forward_join_handle.abort_handle());
        Ok(rx)
    }

    async fn send(&mut self, data: &[u8]) -> JaResult<()> {
        let item = Message::Binary(data.to_vec());
        if let Some(sender) = &mut self.sender {
            sender.send(item).await?;
        } else {
            log::error!("Transport not opened!");
            return Err(JaError::TransportNotOpened);
        }
        Ok(())
    }
}

impl Drop for WebsocketTransport {
    fn drop(&mut self) {
        if let Some(join_handle) = self.abort_handle.take() {
            log::trace!("Dropping wss transport");
            join_handle.abort();
        }
    }
}
