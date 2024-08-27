use std::fmt::Debug;
use std::ops::Deref;

pub trait GenerateTransaction: Send + Sync + Debug + 'static {
    fn generate_transaction(&self) -> String;
}

#[derive(Debug)]
pub struct TransactionGenerator(Box<dyn GenerateTransaction>);

impl TransactionGenerator {
    pub fn new(generator: impl GenerateTransaction) -> Self {
        Self(Box::new(generator))
    }
}

impl Deref for TransactionGenerator {
    type Target = Box<dyn GenerateTransaction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum TransactionGenerationStrategy {
    Random,
}

impl TransactionGenerationStrategy {
    pub fn generator(self) -> impl GenerateTransaction {
        match self {
            TransactionGenerationStrategy::Random => RandomTransactionGenerator,
        }
    }
}

#[derive(Debug)]
struct RandomTransactionGenerator;

impl GenerateTransaction for RandomTransactionGenerator {
    fn generate_transaction(&self) -> String {
        use rand::distributions::Alphanumeric;
        use rand::Rng;

        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect()
    }
}
