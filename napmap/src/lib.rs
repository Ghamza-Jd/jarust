use indexmap::IndexMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::Notify;

/// ## NapMap
///
/// `NapMap` is a `HashMap` that pauses the task that's trying to get the data
/// if the requested data is not available.
pub struct NapMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone + Debug,
{
    map: Arc<RwLock<IndexMap<K, V>>>,
    notifiers: Arc<AsyncMutex<HashMap<K, Arc<Notify>>>>,
}

impl<K, V> NapMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            map: Arc::new(RwLock::new(IndexMap::new())),
            notifiers: Arc::new(AsyncMutex::new(HashMap::new())),
        }
    }

    /// Inserts a key and a value into the map.
    /// And notifies waiting tasks if any.
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self, v))]
    pub async fn insert(&self, k: K, v: V) {
        tracing::trace!("Insert");
        self.map.write().unwrap().insert(k.clone(), v);
        if let Some(notify) = self.notifiers.lock().await.remove(&k) {
            notify.notify_waiters();
            tracing::trace!("Notified all waiting tasks");
        }
    }

    /// Get an immutable reference to an entry in the map.
    /// If the data is already presented return it, else wait until the data is inserted.
    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    pub async fn get(&self, k: K) -> Option<V> {
        tracing::trace!("Get");
        if self.map.read().unwrap().contains_key(&k) {
            tracing::debug!("Contains key");
            return self.map.read().unwrap().get(&k).map(|value| value.clone());
        }

        let mut notifiers = self.notifiers.lock().await;
        let notify = notifiers
            .entry(k.clone())
            .or_insert(Arc::new(Notify::new()))
            .clone();
        drop(notifiers);

        tracing::trace!("Waiting...");
        notify.notified().await;
        tracing::trace!("Notified, data is available");
        self.map.read().unwrap().get(&k).map(|value| value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::NapMap;
    use std::sync::Arc;
    use std::time::Duration;
    use tracing_subscriber::EnvFilter;

    // Add this to a test to see the logs
    fn _tracing_sub() {
        let env_filter =
            EnvFilter::from_default_env().add_directive("jarust=trace".parse().unwrap());
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    }

    #[tokio::test]
    async fn it_should_wait_until_data_is_inserted() {
        let napmap = Arc::new(NapMap::new());

        tokio::spawn({
            let map = napmap.clone();
            async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                map.insert("key", 7).await;
            }
        });

        let res = napmap.get("key").await.unwrap();
        assert_eq!(res, 7);
    }

    #[tokio::test]
    async fn it_should_notify_all_waiters() {
        let napmap = Arc::new(NapMap::new());

        tokio::spawn({
            let map = napmap.clone();
            async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                map.insert("key", 7).await;
            }
        });

        let first_handle = tokio::spawn({
            let map = napmap.clone();
            async move {
                let res = map.get("key").await.unwrap();
                assert_eq!(res, 7);
            }
        });

        let second_handle = tokio::spawn({
            let map = napmap.clone();
            async move {
                let res = map.get("key").await.unwrap();
                assert_eq!(res, 7);
            }
        });

        first_handle.await.unwrap();
        second_handle.await.unwrap();
    }
}
