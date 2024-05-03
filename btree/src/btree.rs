use crate::ordered_map::SimpleOrderedMap;

// todo add any members you want here
pub struct BTree {}

/// todo implement me
impl SimpleOrderedMap for BTree {
    type Key = u64;

    type Value = u64;

    fn new() -> Self {
        todo!()
    }

    fn lookup(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        todo!()
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::{test_ordered_set_functionality, test_ordered_set_reference};

    use super::BTree;

    #[test]
    fn test_btree_functionality() {
        for size in [10, 99, 837, 48329, 384933] {
            test_ordered_set_functionality::<BTree>(size);
        }
    }

    #[test]
    fn test_btree_reference() {
        test_ordered_set_reference::<BTree>();
    }
}
