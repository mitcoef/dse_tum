use std::collections::HashMap;
use std::hash::Hash;

use crate::hashtable::SimpleHashtable;

impl<K: Eq + Hash + Clone, V: Clone> SimpleHashtable for HashMap<K, V> {
    type Key = K;
    type Value = V;

    fn new(size: usize) -> Self {
        HashMap::with_capacity(size)
    }

    fn insert<'a>(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.insert(key, value)
    }

    fn lookup(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    fn erase(&mut self, key: &Self::Key) -> bool {
        self.remove(key).is_some()
    }

    fn rehash(&self, map: &mut impl SimpleHashtable<Key = K, Value = V>) {
        for (k, v) in self.iter() {
            map.insert(k.to_owned(), v.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{test_hashmap_functionality, test_lookup_reference};
    use std::collections::HashMap;

    #[test]
    fn test_functionality_std() {
        for size in [10, 99, 837, 48329, 384933] {
            test_hashmap_functionality::<HashMap<u64, u64>>(size);
        }
    }

    #[test]
    fn test_reference_std() {
        for size in [10, 99, 837, 48329, 384933] {
            test_lookup_reference::<HashMap<u64, u64>>(size)
        }
    }
}
