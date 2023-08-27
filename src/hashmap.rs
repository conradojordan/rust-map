/*
TODO:
    -Add hash algorithm
    -Make CustomHashMap generic and not just implemented for i32 keys and values
*/
mod siphash;
// use rand::Rng;
use siphash::SipHasher;

#[derive(Clone)]
struct Bucket {
    items: Vec<(i32, i32)>,
}

pub struct CustomHashMap {
    hash_key: u128,
    buckets: Vec<Bucket>,
}

impl Bucket {
    fn new() -> Bucket {
        Bucket { items: Vec::new() }
    }

    fn add(&mut self, key: i32, value: i32) {
        self.items.push((key, value));
    }

    fn get(&self, key: i32) -> Option<i32> {
        for item in self.items.iter() {
            if item.0 == key {
                return Some(item.1);
            }
        }
        return None;
    }
}

impl CustomHashMap {
    pub fn new() -> CustomHashMap {
        // let mut rng = rand::thread_rng();
        CustomHashMap {
            // hash_key: rng.gen::<u128>(),
            hash_key: 0x41424344454647486162636465666768 as u128,
            buckets: vec![Bucket::new(); 100],
        }
    }

    pub fn add(&mut self, key: i32, value: i32) {
        let idx = (key.sip_hash(self.hash_key) % self.buckets.len() as u64) as usize;
        self.buckets[idx].add(key, value);
    }

    pub fn get(&self, key: i32) -> Option<i32> {
        let idx = (key.sip_hash(self.hash_key) % self.buckets.len() as u64) as usize;
        self.buckets[idx].get(key)
    }
}
