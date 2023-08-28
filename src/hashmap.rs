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

    pub fn set(&mut self, key: K, value: V) {
        let idx = (key.hash(self.hash_seed) % self.buckets.len() as u64) as usize;
        self.buckets[idx].set(key, value);
    }

    pub fn get(&self, key: K) -> Option<V> {
        if self.len() == 0 {
            None
        } else {
            let idx = (key.hash(self.hash_seed) % self.buckets.len() as u64) as usize;
            self.buckets[idx].get(key)
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}
