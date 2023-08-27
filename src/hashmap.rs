mod siphash;

use rand::Rng;
use siphash::SipHasher;
use std::cmp::PartialEq; // Keys must be of a comparable type

#[derive(Clone)]
struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

pub struct CustomHashMap<K, V> {
    hash_key: u128,
    buckets: Vec<Bucket<K, V>>,
}

impl<K: PartialEq, V: Clone> Bucket<K, V> {
    fn new() -> Bucket<K, V> {
        Bucket { items: Vec::new() }
    }

    fn add(&mut self, key: K, value: V) {
        self.items.push((key, value));
    }

    fn get(&self, key: K) -> Option<V> {
        for item in self.items.iter() {
            if item.0 == key {
                return Some(item.1.clone());
            }
        }
        return None;
    }
}

impl<K: Clone + SipHasher + PartialEq, V: Clone> CustomHashMap<K, V> {
    pub fn new() -> CustomHashMap<K, V> {
        let mut rng = rand::thread_rng();
        CustomHashMap {
            hash_key: rng.gen::<u128>(),
            buckets: vec![Bucket::<K, V>::new(); 100],
        }
    }

    pub fn add(&mut self, key: K, value: V) {
        let idx = (key.sip_hash(self.hash_key) % self.buckets.len() as u64) as usize;
        self.buckets[idx].add(key, value);
    }

    pub fn get(&self, key: K) -> Option<V> {
        let idx = (key.sip_hash(self.hash_key) % self.buckets.len() as u64) as usize;
        self.buckets[idx].get(key)
    }
}
