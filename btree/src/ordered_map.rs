pub trait SimpleOrderedMap {
    type Key;
    type Value;

    fn new() -> Self;

    fn lookup(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}
