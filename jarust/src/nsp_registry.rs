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
    root_nsp: String,
}

#[derive(Debug)]
pub struct Exclusive {
    namespaces: HashMap<String, mpsc::Sender<JaResponse>>,
}

#[derive(Debug)]
pub(crate) struct Inner {
    shared: Shared,
    exclusive: RwLock<Exclusive>,
}

#[derive(Clone, Debug)]
pub(crate) struct NamespaceRegistry(Arc<Inner>);

impl Deref for NamespaceRegistry {
    type Target = Arc<Inner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NamespaceRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl NamespaceRegistry {
    pub(crate) fn new(root_nsp: &str) -> Self {
        let shared = Shared {
            root_nsp: root_nsp.to_string(),
        };
        let exclusive = Exclusive {
            namespaces: HashMap::new(),
        };
        Self(Arc::new(Inner {
            shared,
            exclusive: RwLock::new(exclusive),
        }))
    }

    pub(crate) fn create_namespace(&mut self, namespace: &str) -> mpsc::Receiver<JaResponse> {
        let (tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        {
            self.exclusive
                .write()
                .expect("Failed to acquire write lock")
                .namespaces
                .insert(namespace.into(), tx);
        }
        log::trace!("Namespace created {{ id: {namespace} }}");
        rx
    }

    pub(crate) async fn publish(&self, namespace: &str, message: JaResponse) -> JaResult<()> {
        let channel = {
            let guard = self.exclusive.read().expect("Failed to acquire read lock");
            guard.namespaces.get(namespace).cloned()
        };

        if let Some(channel) = channel {
            if channel.send(message.clone()).await.is_err() {
                return Err(JaError::SendError);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::NamespaceRegistry;
    use crate::japrotocol::JaResponse;
    use crate::japrotocol::JaResponseProtocol;

    #[tokio::test]
    async fn test_1() {
        let mut nsp_registry = NamespaceRegistry::new("janus");
        let mut channel_one = nsp_registry.create_namespace("janus");
        let mut channel_two = nsp_registry.create_namespace("janus/123");

        nsp_registry
            .publish(
                "janus",
                JaResponse {
                    janus: JaResponseProtocol::Ack,
                    transaction: None,
                    session_id: None,
                    sender: None,
                },
            )
            .await
            .unwrap();
        nsp_registry
            .publish(
                "janus",
                JaResponse {
                    janus: JaResponseProtocol::Ack,
                    transaction: None,
                    session_id: None,
                    sender: None,
                },
            )
            .await
            .unwrap();
        nsp_registry
            .publish(
                "janus/123",
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
        let size_one = channel_one.recv_many(&mut buff_one, 10).await;

        let mut buff_two = vec![];
        let size_two = channel_two.recv_many(&mut buff_two, 10).await;

        assert_eq!(size_one, 2);
        assert_eq!(size_two, 1);
    }
}
