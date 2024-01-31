use crate::jaconfig::CHANNEL_BUFFER_SIZE;
use crate::japrotocol::JaResponse;
use crate::prelude::*;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Shared {
    root_path: String,
}

#[derive(Debug)]
pub struct Exclusive {
    routes: HashMap<String, mpsc::Sender<JaResponse>>,
}

#[derive(Debug)]
pub(crate) struct Inner {
    shared: Shared,
    exclusive: RwLock<Exclusive>,
}

#[derive(Clone, Debug)]
pub(crate) struct JaRouter(Arc<Inner>);

impl Deref for JaRouter {
    type Target = Arc<Inner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JaRouter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl JaRouter {
    pub(crate) fn new(root_path: &str) -> (Self, mpsc::Receiver<JaResponse>) {
        let shared = Shared {
            root_path: root_path.to_string(),
        };
        let exclusive = Exclusive {
            routes: HashMap::new(),
        };
        let inner = Arc::new(Inner {
            shared,
            exclusive: RwLock::new(exclusive),
        });
        let mut jarouter = Self(inner);
        let channel = jarouter.make_root_route();
        (jarouter, channel)
    }

    fn make_route(&mut self, path: &str) -> mpsc::Receiver<JaResponse> {
        let (tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        {
            self.exclusive
                .write()
                .expect("Failed to acquire write lock")
                .routes
                .insert(path.into(), tx);
        }
        log::trace!("Route created {{ path: {path} }}");
        rx
    }

    fn make_root_route(&mut self) -> mpsc::Receiver<JaResponse> {
        let path = self.shared.root_path.clone();
        self.make_route(&path)
    }

    pub(crate) fn add_subroute(&mut self, end: &str) -> mpsc::Receiver<JaResponse> {
        let abs_path = &format!("{}/{}", self.shared.root_path, end);
        self.make_route(&abs_path)
    }

    async fn publish(&self, path: &str, message: JaResponse) -> JaResult<()> {
        let channel = {
            let guard = self.exclusive.read().expect("Failed to acquire read lock");
            guard.routes.get(path).cloned()
        };
        if let Some(channel) = channel {
            if channel.send(message.clone()).await.is_err() {
                return Err(JaError::SendError);
            }
        }
        Ok(())
    }

    pub(crate) async fn pub_root(&self, message: JaResponse) -> JaResult<()> {
        let path = self.shared.root_path.clone();
        self.publish(&path, message).await
    }

    pub(crate) async fn pub_subroute(&self, subroute: &str, message: JaResponse) -> JaResult<()> {
        let path = &format!("{}/{}", self.shared.root_path, subroute);
        self.publish(&path, message).await
    }
}

impl JaRouter {
    pub fn root_path(&self) -> String {
        self.shared.root_path.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::JaRouter;
    use crate::japrotocol::JaResponse;
    use crate::japrotocol::JaResponseProtocol;

    #[tokio::test]
    async fn test_basic_usage() {
        let (mut router, mut root_channel) = JaRouter::new("janus");
        let mut channel_one = router.add_subroute("123");

        router
            .pub_root(JaResponse {
                janus: JaResponseProtocol::Ack,
                transaction: None,
                session_id: None,
                sender: None,
            })
            .await
            .unwrap();
        router
            .pub_root(JaResponse {
                janus: JaResponseProtocol::Ack,
                transaction: None,
                session_id: None,
                sender: None,
            })
            .await
            .unwrap();

        router
            .pub_subroute(
                "123",
                JaResponse {
                    janus: JaResponseProtocol::Ack,
                    transaction: None,
                    session_id: None,
                    sender: None,
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
