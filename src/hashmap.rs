/*
TODO:
    -Add hash algorithm
    -Make CustomHashMap generic and not just implemented for i32 keys and values
*/

#[derive(Clone)]
struct Bucket {
    items: Vec<(i32, i32)>,
}

pub struct CustomHashMap {
    buckets: Vec<Bucket>,
}

impl Bucket {
    fn new() -> Bucket {
        Bucket {
            items: Vec::new()
        }
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
        return None
    }
}

impl CustomHashMap {
    pub fn new() -> CustomHashMap {
        CustomHashMap {
            buckets: vec![Bucket::new(); 100],
        }
    }

    pub fn add(&mut self, key: i32, value: i32) {
        let idx = (key % self.buckets.len() as i32) as usize;
        self.buckets[idx].add(key, value);
    }

    pub fn get(&self, key: i32) -> Option<i32> {
        let idx = (key % self.buckets.len() as i32) as usize;
        self.buckets[idx].get(key)
    }
}