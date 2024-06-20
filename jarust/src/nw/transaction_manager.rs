use indexmap::IndexMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub(crate) struct TransactionManager {
    inner: Arc<RwLock<IndexMap<String, String>>>
}

impl TransactionManager {
    #[tracing::instrument(level = tracing::Level::TRACE)]
    pub(crate) fn new() -> Self {
        tracing::debug!("Creating new transaction manager");
        let transactions = IndexMap::new();
        let inner = Arc::new(RwLock::new(transactions));
        Self { inner }
    }

    async fn contains(&self, id: &str) -> bool {
        self.inner.read().await.contains_key(id)
    }

    pub(crate) async fn get(&self, id: &str) -> Option<String> {
        self.inner.read().await.get(id).cloned()
    }

    async fn insert(&self, id: &str, transaction: &str) {
        let mut guard = self.inner.write().await;
        guard.insert(id.into(), transaction.into());
    }

    async fn remove(&self, id: &str) {
        self.inner.write().await.shift_remove(id);
    }

    #[tracing::instrument(parent = None, skip(self))]
    pub(crate) async fn create_transaction(&self, id: &str, path: &str) {
        if self.contains(id).await {
            return;
        }
        self.insert(id, path).await;
        tracing::trace!("Transaction created");
    }

    #[tracing::instrument(parent = None, skip(self))]
    pub(crate) async fn success_close(&self, id: &str) {
        let tx = self.get(id).await;
        if let Some(tx) = tx {
            self.remove(&tx).await;
            tracing::trace!("Transaction closed successfully");
        }
    }
}
