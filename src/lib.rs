pub mod hash_new {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    #[derive(Debug, Clone)]
    pub struct Entry<K, V> {
        key: K,
        value: V,
    }

    pub struct HashMap2<K, V> {
        pub table: Vec<Option<Entry<K, V>>>,
        pub size: usize,
    }

    impl<K, V> HashMap2<K, V>
    where
        K: Hash + Clone,
        V: Clone,
    {
        pub fn new() -> Self {
            let mut vec = vec![None; 1 << 5];
            Self {
                table: vec,
                size: 0 as usize,
            }
        }
        pub fn insert(&mut self, key: K, value: V) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let hash = hasher.finish();
            let mut entry = Some(Entry {
                key: key,
                value: value,
            });
            self.table.insert((hash as usize) % self.table.len(), entry);
            self.size+=1;
            
        }
    }
}
