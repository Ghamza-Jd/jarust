use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct PendingTransaction {
    id: String,
    request: String,
    pub namespace: String,
}

pub struct Inner {
    transactions: HashMap<String, PendingTransaction>,
}

#[derive(Clone)]
pub struct TransactionManager(Arc<RwLock<Inner>>);

impl std::ops::Deref for TransactionManager {
    type Target = Arc<RwLock<Inner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TransactionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TransactionManager {
    pub fn new() -> Self {
        log::trace!("Creating new transaction manager");
        let transactions = HashMap::new();
        Self(Arc::new(RwLock::new(Inner { transactions })))
    }

    fn contains(&self, id: &str) -> bool {
        self.read().unwrap().transactions.contains_key(id)
    }

    pub fn get(&self, id: &str) -> Option<PendingTransaction> {
        self.read().unwrap().transactions.get(id).cloned()
    }

    fn _size(&self) -> usize {
        self.read().unwrap().transactions.len()
    }

    fn insert(&self, id: &str, transaction: PendingTransaction) {
        self.write()
            .unwrap()
            .transactions
            .insert(id.into(), transaction);
    }

    fn remove(&self, id: &str) {
        self.write().unwrap().transactions.remove(id);
    }

    pub fn create_transaction(&self, id: &str, request: &str, namespace: &str) {
        if self.contains(id) {
            return;
        }

        let pending_transaction = PendingTransaction {
            id: id.into(),
            request: request.into(),
            namespace: namespace.into(),
        };

        self.insert(id, pending_transaction);
        log::trace!(
            "Transaction [{}] created in namespace [{}] for request \"{}\"",
            id,
            namespace,
            request
        );
    }

    pub fn success_close(&self, id: &str) {
        let tx = self.get(id);
        if let Some(tx) = tx {
            self.remove(&tx.id);
            log::trace!(
                "Transaction [{}] successfully closed for request \"{}\"",
                tx.id,
                tx.request
            );
        }
    }
}
