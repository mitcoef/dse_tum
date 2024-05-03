use std::collections::VecDeque;
use std::hash::Hasher;

use rustc_hash::FxHasher;

use crate::hashtable::SimpleHashtable;

#[derive(Clone)]
struct Entry {
    key: u64,
    value: u64,
}

/// TODO add any members you need
pub struct ChainingHashtable {
    mask: u64,
    table: Box<[VecDeque<Entry>]>,
}

impl ChainingHashtable {
    fn hashmap_index(&self, key: u64) -> usize {
        let mut hasher = FxHasher::default();
        hasher.write_u64(key);
        (hasher.finish() & self.mask) as usize
    }
}

/// Implement a Hashtable with Chaining, specialized on integer keys and values
impl SimpleHashtable for ChainingHashtable {
    type Key = u64;
    type Value = u64;

    fn new(size: usize) -> Self {
        let size = size.next_power_of_two();
        Self {
            mask: (size - 1) as u64,
            table: vec![VecDeque::new(); size].into_boxed_slice(),
        }
    }

    fn insert<'a>(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        let idx = self.hashmap_index(key);

        let deq = &mut self.table[idx];

        // iterate over entries mutably and edit value when found
        for e in deq.iter_mut() {
            if e.key == key {
                let result = e.value;
                e.value = value;
                return Some(result);
            }
        }

        // key is not yet contained, insert
        deq.push_front(Entry { key, value });

        None
    }

    fn lookup(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        let idx = self.hashmap_index(*key);

        let deq = &mut self.table[idx];

        // iterate over entries mutably and edit value when found
        for e in deq.iter_mut() {
            if e.key == *key {
                return Some(&mut e.value);
            }
        }

        None
    }

    fn erase(&mut self, key: &Self::Key) -> bool {
        let idx = self.hashmap_index(*key);

        let deq = &mut self.table[idx];
        let idx: Option<usize> = {
            let mut idx: usize = 0;
            let mut result = None;
            for e in deq.iter_mut() {
                if e.key == *key {
                    result = Some(idx);
                    break;
                } else {
                    idx += 1;
                }
            }
            result
        };

        match idx {
            None => false,
            Some(idx) => {
                deq.remove(idx);
                true
            }
        }
    }

    fn rehash(&self, map: &mut impl SimpleHashtable<Key = Self::Key, Value = Self::Value>) {
        for dec in self.table.iter() {
            for Entry { key, value } in dec.iter() {
                map.insert(*key, *value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chaining_hashtable::ChainingHashtable;
    use crate::test_utils::{test_hashmap_functionality, test_lookup_reference};

    #[test]
    fn test_chaining_hashtable() {
        for size in [10, 99, 837, 48329, 384933] {
            test_hashmap_functionality::<ChainingHashtable>(size);
        }
    }

    #[test]
    fn test_reference_chaining() {
        for size in [10, 99, 837, 48329, 384933] {
            test_lookup_reference::<ChainingHashtable>(size)
        }
    }
}
