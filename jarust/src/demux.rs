use crate::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::mpsc;

pub struct Inner {
    channels: HashMap<String, mpsc::Sender<String>>,
}

#[derive(Clone)]
pub struct Demux(Arc<RwLock<Inner>>);

impl std::ops::Deref for Demux {
    type Target = Arc<RwLock<Inner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Demux {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Demux {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Inner {
            channels: HashMap::new(),
        })))
    }

    pub fn create_namespace(&mut self, namespace: &str) -> mpsc::Receiver<String> {
        let (tx, rx) = mpsc::channel(10);
        {
            self.write().unwrap().channels.insert(namespace.into(), tx);
        }
        log::trace!("Namespace created: {{ id: {namespace} }}");
        rx
    }

    pub async fn publish(&self, namespace: &str, message: String) -> JaResult<()> {
        let channel = {
            let guard = self.read().unwrap();
            guard.channels.get(namespace).cloned()
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
    use super::Demux;

    #[tokio::test]
    async fn test_1() {
        let mut demux = Demux::new();
        let mut channel_one = demux.create_namespace("janus");
        let mut channel_two = demux.create_namespace("janus/123");

        demux
            .publish("janus", "1st message".to_string())
            .await
            .unwrap();
        demux
            .publish("janus", "2nd message".to_string())
            .await
            .unwrap();
        demux
            .publish("janus/123", "3rd message".to_string())
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
