use crate::ordered_map::SimpleOrderedMap;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn test_ordered_set_functionality<T: SimpleOrderedMap<Key = u64, Value = u64>>(size: usize) {
    let mut tree = T::new();

    // create keys and shuffle
    let mut keys: Vec<u64> = (0..size as u64).collect();
    keys.shuffle(&mut thread_rng());

    // insert with fixed value, ensure there was no previous element
    for key in keys.iter() {
        assert!(tree.insert(*key, 42).is_none());
    }

    // assure previous inserts were successful
    for key in keys.iter() {
        assert!(tree.lookup(key).is_some_and(|e| *e == 42));
    }

    // todo should I also implement this? isn't this leaking too much implementation?
    //   ensure(tree.rootNodeCount() != 0);
    //   ensure(tree.rootNodeCount() <= size);

    // update and ensure that previous value was 42
    for key in keys.iter() {
        assert!(tree.insert(*key, *key).is_some_and(|e| e == 42));
    }

    for key in 0..(size + 100) as u64 {
        let result = tree.lookup(&key);

        if key < size as u64 {
            assert!(result.is_some_and(|e| *e == key))
        } else {
            assert!(result.is_none(), "looked up key {key}")
        }
    }
}

pub fn test_ordered_set_reference<T: SimpleOrderedMap<Key = u64, Value = u64>>() {
    let mut tree = T::new();

    assert!(tree.insert(0, 42).is_none());

    let result = tree.lookup(&0);

    assert!(result.as_ref().is_some_and(|e| **e == 42));

    if let Some(e) = result {
        *e = 1337;
    }

    assert!(tree.lookup(&0).is_some_and(|e| *e == 1337));
}
