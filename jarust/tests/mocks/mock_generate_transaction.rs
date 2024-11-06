use jarust::core::GenerateTransaction;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
struct InnerMockGenerateTransaction {
    next_transaction: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MockGenerateTransaction {
    inner: Arc<Mutex<InnerMockGenerateTransaction>>,
}

impl MockGenerateTransaction {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(InnerMockGenerateTransaction {
                next_transaction: None,
            })),
        }
    }

    #[allow(dead_code)]
    pub fn next_transaction(&mut self, transaction: &str) {
        self.inner.lock().unwrap().next_transaction = Some(transaction.to_string());
    }
}

impl GenerateTransaction for MockGenerateTransaction {
    fn generate_transaction(&self) -> String {
        self.inner
            .lock()
            .unwrap()
            .next_transaction
            .clone()
            .expect("Call next_transaction before use")
    }
}
