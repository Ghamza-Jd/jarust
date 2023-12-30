use crate::jaconfig::CHANNEL_BUFFER_SIZE;
use crate::japrotocol::JaResponse;
use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::mpsc;

pub(crate) struct Inner {
    namespaces: HashMap<String, mpsc::Sender<JaResponse>>,
}

#[derive(Clone)]
pub(crate) struct NamespaceRegistry(Arc<RwLock<Inner>>);

impl std::ops::Deref for NamespaceRegistry {
    type Target = Arc<RwLock<Inner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for NamespaceRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl NamespaceRegistry {
    pub(crate) fn new() -> Self {
        Self(Arc::new(RwLock::new(Inner {
            namespaces: HashMap::new(),
        })))
    }

    pub(crate) fn create_namespace(&mut self, namespace: &str) -> mpsc::Receiver<JaResponse> {
        let (tx, rx) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        {
            self.write()
                .unwrap()
                .namespaces
                .insert(namespace.into(), tx);
        }
        log::trace!("Namespace created {{ id: {namespace} }}");
        rx
    }

    pub(crate) async fn publish(&self, namespace: &str, message: JaResponse) -> JaResult<()> {
        let channel = {
            let guard = self.read().unwrap();
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
        let mut nsp_registry = NamespaceRegistry::new();
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
