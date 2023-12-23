use super::trans::Transport;
use crate::prelude::*;
use async_trait::async_trait;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;

type WebSocketSender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

pub struct WebsocketTransport {
    sender: Option<WebSocketSender>,
}

#[async_trait]
impl Transport for WebsocketTransport {
    fn new() -> Self {
        Self { sender: None }
    }

    async fn connect(&mut self, uri: &str) -> JaResult<mpsc::Receiver<String>> {
        let mut request = uri.into_client_request()?;
        let headers = request.headers_mut();
        headers.insert("Sec-Websocket-Protocol", "janus-protocol".parse()?);
        let (stream, _) = connect_async(request).await?;
        let (sender, mut receiver) = stream.split();
        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                if let Message::Text(text) = message {
                    tx.send(text).await.unwrap();
                }
            }
        });

        self.sender = Some(sender);
        Ok(rx)
    }

    async fn send(&mut self, data: &[u8]) -> JaResult<()> {
        let item = Message::Binary(data.to_vec());
        if let Some(sender) = &mut self.sender {
            sender.send(item).await?;
        }
        Ok(())
    }
}
