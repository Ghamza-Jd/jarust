use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct CircularBuffer<K, V> {
    map: HashMap<K, V>,
    keys: VecDeque<K>,
    capacity: usize,
}

impl<K, V> CircularBuffer<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
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
    fn test_circular_buffer() {
        let mut buffer = CircularBuffer::new(3);
        buffer.put("a", 1);
        buffer.put("b", 2);
        buffer.put("c", 3);
        buffer.put("d", 4);
        assert_eq!(buffer.get(&"a"), None);
        assert_eq!(buffer.get(&"b"), Some(&2));
        assert_eq!(buffer.get(&"c"), Some(&3));
        assert_eq!(buffer.get(&"d"), Some(&4));
    }
}
