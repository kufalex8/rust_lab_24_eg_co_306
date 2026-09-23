use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    capacity: usize,
    map: HashMap<K, V>,
    order: Vec<K>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            map: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        }

        self.map.insert(key.clone(), value);
        self.order.push(key.clone());

        if self.map.len() > self.capacity {
            let oldest = self.order.remove(0);
            self.map.remove(&oldest);
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push(key.clone());
        }

        self.map.get(key)
    }
}

pub fn run() {
    let mut cache = Cache::new(3);

    cache.insert("A", 10);
    cache.insert("B", 20);
    cache.insert("C", 30);

    println!("A = {:?}", cache.get(&"A"));

    cache.insert("D", 40);

    println!("A = {:?}", cache.get(&"A"));
    println!("B = {:?}", cache.get(&"B"));
    println!("C = {:?}", cache.get(&"C"));
    println!("D = {:?}", cache.get(&"D"));
}
