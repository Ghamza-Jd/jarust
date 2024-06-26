use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct RingBufMap<K, V> {
    map: HashMap<K, V>,
    keys: VecDeque<K>,
    capacity: usize,
}

impl<K, V> RingBufMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity should be > 0");
        Self {
            map: HashMap::with_capacity(capacity),
            keys: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.keys.len() == self.capacity {
            if let Some(oldest_key) = self.keys.pop_front() {
                self.map.remove(&oldest_key);
            }
        }
        self.keys.push_back(key.clone());
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_overwrite_the_first_inserted_val() {
        let mut buffer = RingBufMap::new(3);
        buffer.put("a", 1);
        buffer.put("b", 2);
        buffer.put("c", 3);
        buffer.put("d", 4);
        assert_eq!(buffer.get(&"a"), None);
        assert_eq!(buffer.get(&"b"), Some(&2));
        assert_eq!(buffer.get(&"c"), Some(&3));
        assert_eq!(buffer.get(&"d"), Some(&4));
    }

    #[test]
    #[should_panic]
    fn it_should_panic_on_passing_zero() {
        RingBufMap::<String, String>::new(0);
    }
}
