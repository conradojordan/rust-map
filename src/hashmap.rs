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
    fn hash(&self, hash_seed: u128) -> u64;
}

#[derive(Clone)]
struct Bucket<K, V> {
    items: Vec<(K, V)>,
}

pub struct CJHashMap<K, V> {
    size: usize,
    hash_seed: u128,
    buckets: Vec<Bucket<K, V>>,
}

impl<K: PartialEq, V: Clone> Bucket<K, V> {
    fn new() -> Bucket<K, V> {
        Bucket { items: Vec::new() }
    }

    fn set(&mut self, key: K, value: V) {
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

    fn remove(&mut self, key: K) -> Option<V> {
        let mut found = None;
        let mut idx: usize = 0;
        for (i, item) in self.items.iter().enumerate() {
            if item.0 == key {
                idx = i;
                found = Some(item.1.clone());
                break;
            }
        }
        if found.is_some() {
            self.items.swap_remove(idx);
            found
        } else {
            None
        }
    }
}

impl<K: Clone + MapKeyHasher + PartialEq, V: Clone> CJHashMap<K, V> {
    pub fn new() -> CJHashMap<K, V> {
        let mut rng = rand::thread_rng();
        CJHashMap {
            size: 0,
            hash_seed: rng.gen::<u128>(),
            buckets: vec![Bucket::<K, V>::new(); NUM_BUCKETS],
        }
    }

    fn get_bucket_idx(&self, key: &K) -> usize {
        (key.hash(self.hash_seed) % self.buckets.len() as u64) as usize
    }

    pub fn set(&mut self, key: K, value: V) {
        let idx = self.get_bucket_idx(&key);
        self.buckets[idx].set(key, value);
        self.size += 1;
    }

    pub fn get(&self, key: K) -> Option<V> {
        if self.len() == 0 {
            None
        } else {
            let idx = self.get_bucket_idx(&key);
            self.buckets[idx].get(key)
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let idx = self.get_bucket_idx(&key);
        let removed = self.buckets[idx].remove(key);
        if removed.is_some() {
            self.size -= 1;
        }
        removed
    }

    pub fn len(&self) -> usize {
        self.size
    }
}
