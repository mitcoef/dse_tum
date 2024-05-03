pub trait SimpleHashtable {
    type Key;
    type Value;

    /// Constructs a new hashtable.
    fn new(size: usize) -> Self;

    /// Insert `key` with `value` into hashmap. If `key` was already present, return the previously
    /// associated value as `Some`, otherwise return `None`.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    /// Obtain a mutable reference to the value saved in the hashmap associated with `key`, if
    /// it exists.
    fn lookup(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    /// Erase the entry associated with `key` and return `true` if it exists, otherwise return
    /// `false`.
    fn erase(&mut self, key: &Self::Key) -> bool;

    /// Rehash the hashmap into the argument map referred to by `map`. If `Self::Value`
    /// doesn't implement `Copy`, elements will have to be cloned, because ownership of `self`
    /// is not taken here.
    fn rehash(&self, map: &mut impl SimpleHashtable<Key = Self::Key, Value = Self::Value>);
}
