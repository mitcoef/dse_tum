use crate::hashtable::SimpleHashtable;
use rustc_hash::FxHashMap;
use std::hash::Hash;

pub struct FxWrapper<K, V> {
    map: FxHashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V: Clone> SimpleHashtable for FxWrapper<K, V> {
    type Key = K;
    type Value = V;

    fn new(size: usize) -> Self {
        let mut map = FxHashMap::default();
        map.reserve(size);

        Self { map }
    }

    fn insert<'a>(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.map.insert(key, value)
    }

    fn lookup(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        self.map.get_mut(key)
    }

    fn erase(&mut self, key: &Self::Key) -> bool {
        self.map.remove(key).is_some()
    }

    fn rehash(&self, map: &mut impl SimpleHashtable<Key = K, Value = V>) {
        for (k, v) in self.map.iter() {
            map.insert(k.to_owned(), v.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FxWrapper;
    use crate::test_utils::{test_hashmap_functionality, test_lookup_reference};

    #[test]
    fn test_functionality_fx() {
        for size in [10, 99, 837, 48329, 384933] {
            test_hashmap_functionality::<FxWrapper<u64, u64>>(size);
        }
    }

    #[test]
    fn test_reference_fx() {
        for size in [10, 99, 837, 48329, 384933] {
            test_lookup_reference::<FxWrapper<u64, u64>>(size)
        }
    }
}
