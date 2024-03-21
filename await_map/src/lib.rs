use dashmap::DashMap;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::Notify;

pub struct AwaitMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    map: Arc<DashMap<K, V>>,
    notifiers: AsyncMutex<DashMap<K, Arc<Notify>>>,
}

impl<K, V> AwaitMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            map: Arc::new(DashMap::new()),
            notifiers: AsyncMutex::new(DashMap::new()),
        }
    }

    pub async fn insert(&self, k: K, v: V) {
        self.map.insert(k.clone(), v);
        if let Some(notify) = self.notifiers.lock().await.remove(&k) {
            notify.1.notify_one();
        }
    }

    pub async fn get(&self, k: K) -> Option<V> {
        if self.map.contains_key(&k) {
            return self.map.get(&k).map(|entry| entry.value().clone());
        }

        let notify = Arc::new(Notify::new());

        let notifiers = self.notifiers.lock().await;
        notifiers.entry(k.clone()).or_insert(notify.clone());
        drop(notifiers);

        notify.notified().await;
        self.map.get(&k).map(|entry| entry.value().clone())
    }
}

impl<K, V> Default for AwaitMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::AwaitMap;
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
