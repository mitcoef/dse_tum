use crate::hashtable::SimpleHashtable;

/// This test is specialized on integer keys and values
#[cfg(test)]
pub fn test_hashmap_functionality<T: SimpleHashtable<Key = u64, Value = u64>>(size: usize) {
    let mut map = T::new(size);

    // this is a NOP, just for the compiler
    let size = size as u64;

    // bulk insert values
    for i in 0..size {
        // assure there was no previous value associated with the key
        assert!(map.insert(i, 42).is_none());
    }

    // bulk update values
    for i in 0..size {
        // assure previous value was exactly 42
        assert!(map.insert(i, i).is_some_and(|value| value == 42));
    }

    // lookup values
    for i in 0..size {
        // ensure we got a non-null reference and it's value is equal to the previously updated `i`
        assert!(map.lookup(&i).is_some_and(|value| *value == i));
    }

    // test rehash
    {
        let mut map2 = T::new(size as usize);

        map.rehash(&mut map2);

        // check if values are equal
        for i in 0..size {
            // ensure we still got a non-null reference and it's value is equal to the previously
            // updated `i`
            assert!(map2.lookup(&i).is_some_and(|value| *value == i));
        }
    }

    // erase every third element up to half of size
    for i in (0..size / 2).step_by(3) {
        // ensure deletions are successful
        assert!(map.erase(&i));
    }

    // erase twice
    for i in (0..size / 2).step_by(3) {
        // ensure deletions are successful
        assert!(!map.erase(&i));
    }

    // lookup values
    for i in 0..size / 2 {
        if i % 3 == 0 {
            // these values should have been deleted
            assert!(map.lookup(&i).is_none())
        } else {
            // these values should be unchanged
            assert!(map.lookup(&i).is_some_and(|value| *value == i));
        }
    }

    // erase remaining elements of first half
    for i in 0..size / 2 {
        if i % 3 == 0 {
            // these values should have been deleted
            assert!(!map.erase(&i))
        } else {
            // these values should be unchanged
            assert!(map.erase(&i));
        }
    }
}

/// Ensure that the reference returned to by lookup is actually pointing to the specific entry,
/// i.e. modifying its contents will actually modify the map.
pub fn test_lookup_reference<T: SimpleHashtable<Key = u64, Value = u64>>(size: usize) {
    let mut map = T::new(size);

    assert!(map.insert(0, 42).is_none());

    let entry = map.lookup(&0);

    assert!(entry.as_ref().is_some_and(|e| **e == 42));

    if let Some(e) = entry {
        *e = 1337;
    }

    let entry = map.lookup(&0);

    assert!(entry.is_some_and(|e| *e == 1337));
}
