use crate::ringbuf_map::RingBufMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub(crate) struct TransactionManager {
    inner: Arc<RwLock<RingBufMap<String, String>>>,
}

impl TransactionManager {
    #[tracing::instrument(level = tracing::Level::TRACE)]
    pub(crate) fn new(capacity: usize) -> Self {
        tracing::debug!("Creating new transaction manager");
        let transactions = RingBufMap::new(capacity);
        let inner = Arc::new(RwLock::new(transactions));
        Self { inner }
    }

    pub(crate) async fn get(&self, id: &str) -> Option<String> {
        self.inner.read().await.get(&id.into()).cloned()
    }

    #[tracing::instrument(parent = None, skip(self))]
    pub async fn insert(&self, id: &str, transaction: &str) {
        self.inner.write().await.put(id.into(), transaction.into());
    }
}
