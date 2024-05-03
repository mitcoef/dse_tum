use std::collections::BTreeMap;

use crate::ordered_map::SimpleOrderedMap;

impl<K: Ord + Clone, V: Clone> SimpleOrderedMap for BTreeMap<K, V> {
    type Key = K;

    type Value = V;

    fn new() -> Self {
        BTreeMap::new()
    }

    fn lookup(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.insert(key, value)
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use crate::test_utils::{test_ordered_set_functionality, test_ordered_set_reference};

    #[test]
    fn test_std_functionality() {
        for size in [10, 99, 837, 48329, 384933] {
            test_ordered_set_functionality::<BTreeMap<u64, u64>>(size);
        }
    }

    #[test]
    fn test_std_reference() {
        test_ordered_set_reference::<BTreeMap<u64, u64>>();
    }
}
