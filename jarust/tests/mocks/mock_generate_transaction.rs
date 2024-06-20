use jarust::transaction_gen::GenerateTransaction;

#[derive(Debug)]
pub struct MockGenerateTransaction {
    next_transaction: Option<String>,
}

impl MockGenerateTransaction {
    pub fn new() -> Self {
        Self { next_transaction: None }
    }

    pub fn next_transaction(&mut self, transaction: &str) {
        self.next_transaction = Some(transaction.to_string());
    }
}

impl GenerateTransaction for MockGenerateTransaction {
    fn generate_transaction(&self) -> String {
        self.next_transaction.clone().expect("Call next_transaction before use")
    }
}