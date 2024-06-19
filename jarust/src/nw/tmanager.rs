use indexmap::IndexMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub(crate) struct PendingTransaction {
    pub id: String,
    pub path: String,
}

#[derive(Debug)]
struct Shared {
    bound: usize,
}

#[derive(Debug)]
struct Exclusive {
    transactions: IndexMap<String, PendingTransaction>,
}

#[derive(Debug)]
struct InnerTransactionManager {
    shared: Shared,
    exclusive: RwLock<Exclusive>,
}

#[derive(Clone, Debug)]
pub(crate) struct TransactionManager(Arc<InnerTransactionManager>);

impl TransactionManager {
    #[tracing::instrument(level = tracing::Level::TRACE)]
    pub(crate) fn new(buffer: usize) -> Self {
        assert!(buffer > 0, "Requires buffer > 0");
        tracing::debug!("Creating new transaction manager");
        let transactions = IndexMap::with_capacity(buffer);
        let inner = InnerTransactionManager {
            shared: Shared { bound: buffer },
            exclusive: RwLock::new(Exclusive { transactions }),
        };
        Self(Arc::new(inner))
    }

    async fn contains(&self, id: &str) -> bool {
        self.0.exclusive.read().await.transactions.contains_key(id)
    }

    pub(crate) async fn get(&self, id: &str) -> Option<PendingTransaction> {
        self.0.exclusive.read().await.transactions.get(id).cloned()
    }

    async fn _len(&self) -> usize {
        self.0.exclusive.read().await.transactions.len()
    }

    async fn insert(&self, id: &str, transaction: PendingTransaction) {
        let mut guard = self.0.exclusive.write().await;
        if guard.transactions.len() >= self.0.shared.bound {
            guard.transactions.pop();
        }
        guard.transactions.insert(id.into(), transaction);
    }

    async fn remove(&self, id: &str) {
        self.0.exclusive.write().await.transactions.shift_remove(id);
    }

    #[tracing::instrument(parent = None, skip(self))]
    pub(crate) async fn create_transaction(&self, id: &str, path: &str) {
        if self.contains(id).await {
            return;
        }

        let pending_transaction = PendingTransaction {
            id: id.into(),
            path: path.into(),
        };

        self.insert(id, pending_transaction).await;
        tracing::trace!("Transaction created");
    }

    #[tracing::instrument(parent = None, skip(self))]
    pub(crate) async fn success_close(&self, id: &str) {
        let tx = self.get(id).await;
        if let Some(tx) = tx {
            self.remove(&tx.id).await;
            tracing::trace!("Transaction closed successfully");
        }
    }
}
