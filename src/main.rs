mod hashmap;

use hashmap::CJHashMap;

fn main() {
    println!("CJHashMap<u32, String> hashmap example");
    let mut hashmap: CJHashMap<u32, String> = CJHashMap::new();

    // Adding elements
    let (key, value) = (48u32, String::from("Hello"));
    println!("Adding value '{value}' for key {key}");
    hashmap.set(key, value);
    let (key, value) = (32u32, String::from("Good bye"));
    println!("Adding value '{value}' for key {key}");
    hashmap.set(key, value);

    // Retrieving element
    println!("Searching for value stored in key {key}");
    match hashmap.get(key) {
        None => println!("Element not found"),
        Some(result) => println!("Key {} contains value '{}'", key, result),
    }

    // Getting the current number of elements on the map
    println!("Hashmap has {} items", hashmap.len());
}
