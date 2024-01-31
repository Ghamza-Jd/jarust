use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone, Debug)]
pub(crate) struct PendingTransaction {
    pub id: String,
    request: String,
    pub namespace: String,
}

#[derive(Debug)]
pub(crate) struct Inner {
    transactions: HashMap<String, PendingTransaction>,
}

#[derive(Clone, Debug)]
pub(crate) struct TransactionManager(Arc<RwLock<Inner>>);

impl Deref for TransactionManager {
    type Target = Arc<RwLock<Inner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TransactionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TransactionManager {
    pub(crate) fn new() -> Self {
        log::trace!("Creating new transaction manager");
        let transactions = HashMap::new();
        Self(Arc::new(RwLock::new(Inner { transactions })))
    }

    fn contains(&self, id: &str) -> bool {
        self.read()
            .expect("Failed to aquire read lock")
            .transactions
            .contains_key(id)
    }

    pub(crate) fn get(&self, id: &str) -> Option<PendingTransaction> {
        self.read()
            .expect("Failed to aquire read lock")
            .transactions
            .get(id)
            .cloned()
    }

    fn _size(&self) -> usize {
        self.read()
            .expect("Failed to aquire read lock")
            .transactions
            .len()
    }

    fn insert(&self, id: &str, transaction: PendingTransaction) {
        self.write()
            .expect("Failed to aquire write lock")
            .transactions
            .insert(id.into(), transaction);
    }

    fn remove(&self, id: &str) {
        self.write()
            .expect("Failed to aquire write lock")
            .transactions
            .remove(id);
    }

    pub(crate) fn create_transaction(&self, id: &str, request: &str, namespace: &str) {
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
            "Transaction created {{ id: {id}, namespace: {namespace}, request: {request} }}"
        );
    }

    pub(crate) fn success_close(&self, id: &str) {
        let tx = self.get(id);
        if let Some(tx) = tx {
            self.remove(&tx.id);
            log::trace!(
                "Transaction closed successfully {{ id: {}, namespace: {}, request: {} }}",
                tx.id,
                tx.namespace,
                tx.request
            );
        }
    }
}
