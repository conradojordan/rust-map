/*
    Hashmap implemented as an array of `NUM_BUCKETS` Buckets. Each bucket consists of
    a vector of (key, value) tuples. K and V represent the types of the key and the value
    respectively.
*/

mod siphash;

use rand::Rng;
use std::cmp::PartialEq; // Keys must be of a comparable type

const NUM_BUCKETS: usize = 100;

// A type that will be used as a key to the hash map should implement
// the MapKeyHasher
pub trait MapKeyHasher {
    fn hash(&self, hash_key: u128) -> u64;
}

#[derive(Clone)]
struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

pub struct CJHashMap<K, V> {
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

impl<K: Clone + MapKeyHasher + PartialEq, V: Clone> CJHashMap<K, V> {
    pub fn new() -> CJHashMap<K, V> {
        let mut rng = rand::thread_rng();
        CJHashMap {
            hash_key: rng.gen::<u128>(),
            buckets: vec![Bucket::<K, V>::new(); NUM_BUCKETS],
        }
    }

    pub fn add(&mut self, key: K, value: V) {
        let idx = (key.hash(self.hash_key) % self.buckets.len() as u64) as usize;
        self.buckets[idx].add(key, value);
    }

    pub fn get(&self, key: K) -> Option<V> {
        let idx = (key.hash(self.hash_key) % self.buckets.len() as u64) as usize;
        self.buckets[idx].get(key)
    }
}
