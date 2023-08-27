mod hashmap;

use hashmap::CJHashMap;

fn main() {
    println!("<i32, i32> hashmap example");
    let mut hashmap1: CJHashMap<i32, i32> = CJHashMap::new();

    let (key, value) = (97i32, 8i32);
    hashmap1.add(key, value);
    match hashmap1.get(key) {
        None => println!("Element not found"),
        Some(result) => println!("Key {} contains value {}", key, result),
    }

    println!("\n<u32, String> hashmap example");
    let mut hashmap2: CJHashMap<u32, String> = CJHashMap::new();

    let (key, value) = (48u32, String::from("Hello"));
    hashmap2.add(key, value);
    match hashmap2.get(key) {
        None => println!("Element not found"),
        Some(result) => println!("Key {} contains value {}", key, result),
    }
}
