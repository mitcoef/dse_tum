use std::hash::Hasher;

use rustc_hash::FxHasher;

use crate::hashtable::SimpleHashtable;

#[derive(Clone)]
struct Entry {
    key: u64,
    value: u64,
}

#[derive(Clone)]
enum TableEntry {
    Tombstone,
    Entry(Entry),
}

impl TableEntry {
    fn extract_value(&mut self) -> Option<&mut u64> {
        match self {
            Self::Entry(e) => Some(&mut e.value),
            _ => None,
        }
    }

    fn copy_value(&mut self) -> Option<u64> {
        match self {
            Self::Entry(e) => Some(e.value),
            _ => None,
        }
    }
}

pub struct OpenHashtable {
    mask: u64,
    table: Box<[Option<TableEntry>]>,
}

impl OpenHashtable {
    fn hash(&self, key: u64) -> u64 {
        let mut hasher = FxHasher::default();
        hasher.write_u64(key);
        hasher.finish()
    }

    fn lookup(&mut self, key: u64) -> Option<&mut TableEntry> {
        let hash = self.hash(key);
        let hash_index = (hash & self.mask) as usize;

        // split up the iterator to enable wrap-around
        let (wrap, init) = self.table.split_at_mut(hash_index);

        for entry in init.iter_mut().chain(wrap.iter_mut()) {
            match entry {
                // empty value, so value is not contained
                None => {
                    return None;
                }

                // found the key
                Some(TableEntry::Entry(e)) if e.key == key => {
                    return entry.as_mut();
                }

                // either entry with different key or tombstone, keep searching
                _ => (),
            }
        }

        // searched the whole map but found nothing
        None
    }
}

/// Implement an open Hashtable , specialized on integer keys and values
impl SimpleHashtable for OpenHashtable {
    type Key = u64;
    type Value = u64;

    fn new(size: usize) -> Self {
        let size = size.next_power_of_two();
        Self {
            mask: (size - 1) as u64,
            table: vec![None; size].into_boxed_slice(),
        }
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        let hash = self.hash(key);
        let hash_index = (hash & self.mask) as usize;

        // split up the iterator to enable wrap-around
        let (wrap, init) = self.table.split_at_mut(hash_index);

        for entry in init.iter_mut().chain(wrap.iter_mut()) {
            match entry {
                // not our value, keep searching
                Some(TableEntry::Entry(e)) if e.key != key => {}

                // empty, insert here
                None => {
                    *entry = Some(TableEntry::Entry(Entry { key, value }));
                    return None;
                }
                // either our element or a Tombstone, insert here
                Some(e) => {
                    let ret = e.copy_value();
                    *entry = Some(TableEntry::Entry(Entry { key, value }));
                    return ret;
                }
            }
        }

        // todo hashtable is full here, need to rehash or change signature to an result
        panic!("Hashtable is out of memory");
    }

    fn lookup(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        self.lookup(*key).and_then(|e| e.extract_value())
    }

    fn erase(&mut self, key: &Self::Key) -> bool {
        self.lookup(*key)
            .map(|e| {
                *e = TableEntry::Tombstone;
            })
            .is_some()
    }

    fn rehash(&self, map: &mut impl SimpleHashtable<Key = Self::Key, Value = Self::Value>) {
        for e in self.table.iter().flatten() {
            if let TableEntry::Entry(Entry { key, value }) = e {
                map.insert(*key, *value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::open_hashtable::OpenHashtable;
    use crate::test_utils::{test_hashmap_functionality, test_lookup_reference};

    #[test]
    fn test_open_hashtable() {
        for size in [10, 99, 837, 48329, 384933] {
            test_hashmap_functionality::<OpenHashtable>(size);
        }
    }

    #[test]
    fn test_reference_open() {
        for size in [10, 99, 837, 48329, 384933] {
            test_lookup_reference::<OpenHashtable>(size)
        }
    }
}
