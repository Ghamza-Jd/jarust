use crate::prelude::*;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
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
// type WebSocketReceiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub(crate) struct WebsocketTransport {
    sender: WebSocketSender,
}

impl WebsocketTransport {
    pub async fn connect(uri: &str) -> JaResult<(Self, mpsc::Receiver<String>)> {
        let mut request = uri.into_client_request()?;
        let headers = request.headers_mut();
        headers.insert("Sec-Websocket-Protocol", "janus-protocol".parse()?);
        let (stream, _) = connect_async(request).await?;
        let (sender, mut receiver) = stream.split();
        let (tx, rx) = mpsc::channel(32);
        tokio::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                match message {
                    Message::Text(text) => {
                        tx.send(text).await.unwrap();
                    }
                    _ => {}
                }
            }
        });

        Ok((Self { sender }, rx))
    }

    pub(crate) async fn send(&mut self, message: &str) -> JaResult<()> {
        let item = Message::Text(message.to_string());
        self.sender.send(item).await?;
        Ok(())
    }
}
