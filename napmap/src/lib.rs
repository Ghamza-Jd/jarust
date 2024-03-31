pub mod bounded;
pub mod unbounded;

pub use bounded::NapMap;
pub use unbounded::UnboundedNapMap;

use std::fmt::Debug;
use std::hash::Hash;

pub fn unbounded<K, V>() -> UnboundedNapMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone + Debug,
{
    UnboundedNapMap::new()
}

pub fn napmap<K, V>(buffer: usize) -> NapMap<K, V>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone + Debug,
{
    NapMap::new(buffer)
}
