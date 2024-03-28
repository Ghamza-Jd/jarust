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
            notify.1.notify_one();
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

        let notify = Arc::new(Notify::new());

        let notifiers = self.notifiers.lock().await;
        notifiers.entry(k.clone()).or_insert(notify.clone());
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

    #[tokio::test]
    async fn it_should_wait_on_get_until_data_is_present() {
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
}
