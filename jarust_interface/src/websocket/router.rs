use crate::error::Error;
use crate::japrotocol::JaResponse;
use crate::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

#[derive(Debug)]
struct Shared {
    root_path: String,
}

#[derive(Debug)]
struct Exclusive {
    routes: HashMap<String, mpsc::UnboundedSender<JaResponse>>,
}

#[derive(Debug)]
struct InnerRouter {
    shared: Shared,
    exclusive: RwLock<Exclusive>,
}

#[derive(Clone, Debug)]
pub(crate) struct Router {
    inner: Arc<InnerRouter>,
}

impl Router {
    #[tracing::instrument(level = tracing::Level::TRACE)]
    pub(crate) async fn new(root_path: &str) -> (Self, mpsc::UnboundedReceiver<JaResponse>) {
        let shared = Shared {
            root_path: root_path.to_string(),
        };
        let exclusive = Exclusive {
            routes: HashMap::new(),
        };
        let inner = Arc::new(InnerRouter {
            shared,
            exclusive: RwLock::new(exclusive),
        });
        let mut jarouter = Self { inner };
        let channel = jarouter.make_root_route().await;
        tracing::trace!("created new JaRouter");
        (jarouter, channel)
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    async fn make_route(&mut self, path: &str) -> mpsc::UnboundedReceiver<JaResponse> {
        let (tx, rx) = mpsc::unbounded_channel();
        {
            self.inner
                .exclusive
                .write()
                .await
                .routes
                .insert(path.into(), tx);
        }
        tracing::trace!("New route created");
        rx
    }

    async fn make_root_route(&mut self) -> mpsc::UnboundedReceiver<JaResponse> {
        let path = self.inner.shared.root_path.clone();
        self.make_route(&path).await
    }

    pub(crate) async fn add_subroute(&mut self, end: &str) -> mpsc::UnboundedReceiver<JaResponse> {
        let path = &format!("{}/{}", self.inner.shared.root_path, end);
        self.make_route(path).await
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self, message))]
    async fn publish(&self, path: &str, message: JaResponse) -> Result<()> {
        let channel = {
            let guard = self.inner.exclusive.read().await;
            guard.routes.get(path).cloned()
        };
        if let Some(channel) = channel {
            if channel.send(message.clone()).is_err() {
                return Err(Error::SendError);
            }
        }
        tracing::trace!("Published");
        Ok(())
    }

    pub(crate) async fn pub_root(&self, message: JaResponse) -> Result<()> {
        let path = self.inner.shared.root_path.clone();
        self.publish(&path, message).await
    }

    pub(crate) async fn pub_subroute(&self, subroute: &str, message: JaResponse) -> Result<()> {
        let path = &format!("{}/{}", self.inner.shared.root_path, subroute);
        self.publish(path, message).await
    }
}

impl Router {
    pub fn root_path(&self) -> String {
        self.inner.shared.root_path.clone()
    }

    pub fn path_from_request(request: &Value) -> Option<String> {
        if let (Some(session_id), Some(handle_id)) = (
            request["session_id"].as_u64(),
            request["handle_id"].as_u64(),
        ) {
            Some(format!("{session_id}/{handle_id}"))
        } else {
            request["session_id"]
                .as_u64()
                .map(|session_id| format!("{session_id}"))
        }
    }

    pub fn path_from_response(response: JaResponse) -> Option<String> {
        let session_id = response.session_id?;
        let Some(sender) = response.sender else {
            return Some(format!("{session_id}"));
        };
        Some(format!("{session_id}/{sender}"))
    }
}

#[cfg(test)]
mod tests {
    use super::Router;
    use crate::japrotocol::JaResponse;
    use crate::japrotocol::ResponseType;

    #[tokio::test]
    async fn test_basic_usage() {
        let (mut router, mut root_channel) = Router::new("janus").await;
        let mut channel_one = router.add_subroute("123").await;

        router
            .pub_root(JaResponse {
                janus: ResponseType::Ack,
                transaction: None,
                session_id: None,
                sender: None,
                establishment_protocol: None,
            })
            .await
            .unwrap();
        router
            .pub_root(JaResponse {
                janus: ResponseType::Ack,
                transaction: None,
                session_id: None,
                sender: None,
                establishment_protocol: None,
            })
            .await
            .unwrap();

        router
            .pub_subroute(
                "123",
                JaResponse {
                    janus: ResponseType::Ack,
                    transaction: None,
                    session_id: None,
                    sender: None,
                    establishment_protocol: None,
                },
            )
            .await
            .unwrap();

        let mut buff_one = vec![];
        let size_one = root_channel.recv_many(&mut buff_one, 10).await;

        let mut buff_two = vec![];
        let size_two = channel_one.recv_many(&mut buff_two, 10).await;

        assert_eq!(size_one, 2);
        assert_eq!(size_two, 1);
    }
}
