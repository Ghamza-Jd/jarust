use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate_transaction() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}
