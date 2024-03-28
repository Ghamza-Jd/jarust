use dashmap::DashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::Notify;

pub struct AwaitMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone + Debug,
{
    map: Arc<DashMap<K, V>>,
    notifiers: AsyncMutex<DashMap<K, Arc<Notify>>>,
}

impl<K, V> AwaitMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            map: Arc::new(DashMap::new()),
            notifiers: AsyncMutex::new(DashMap::new()),
        }
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    pub async fn insert(&self, k: K, v: V) {
        tracing::debug!("Insert");
        self.map.insert(k.clone(), v);
        if let Some(notify) = self.notifiers.lock().await.remove(&k) {
            notify.1.notify_waiters();
            tracing::debug!("Notify a sleeping task");
        }
    }

    #[tracing::instrument(level = tracing::Level::TRACE, skip(self))]
    pub async fn get(&self, k: K) -> Option<V> {
        tracing::debug!("Get");
        if self.map.contains_key(&k) {
            tracing::debug!("Contains key");
            return self.map.get(&k).map(|entry| entry.value().clone());
        }

        let notifiers = self.notifiers.lock().await;
        let notify = notifiers
            .entry(k.clone())
            .or_insert(Arc::new(Notify::new()))
            .clone();
        drop(notifiers);

        tracing::debug!("Sleep until notified");
        notify.notified().await;
        tracing::debug!("Notified");
        self.map.get(&k).map(|entry| entry.value().clone())
    }
}

impl<K, V> Default for AwaitMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::AwaitMap;
    use std::sync::Arc;
    use std::time::Duration;
    use tracing_subscriber::EnvFilter;

    // Add this to a test to see the logs
    fn tracing_sub() {
        let env_filter =
            EnvFilter::from_default_env().add_directive("jarust=trace".parse().unwrap());
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    }

    #[tokio::test]
    async fn it_should_wait_until_data_got_inserted() {
        let await_map = Arc::new(AwaitMap::new());

        tokio::spawn({
            let map = await_map.clone();
            async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                map.insert("key", 7).await;
            }
        });

        let res = await_map.get("key").await.unwrap();
        assert_eq!(res, 7);
    }

    #[tokio::test]
    async fn it_should_notify_all_waiters() {
        tracing_sub();
        let await_map = Arc::new(AwaitMap::new());

        tokio::spawn({
            let map = await_map.clone();
            async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                map.insert("key", 7).await;
            }
        });

        let first_handle = tokio::spawn({
            let map = await_map.clone();
            async move {
                let res = map.get("key").await.unwrap();
                assert_eq!(res, 7);
            }
        });

        let second_handle = tokio::spawn({
            let map = await_map.clone();
            async move {
                let res = map.get("key").await.unwrap();
                assert_eq!(res, 7);
            }
        });

        first_handle.await.unwrap();
        second_handle.await.unwrap();
    }
}
