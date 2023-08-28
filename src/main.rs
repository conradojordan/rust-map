mod hashmap;

use hashmap::CJHashMap;

fn main() {
    println!("CJHashMap<u32, String> hashmap example");
    let mut hashmap: CJHashMap<u32, String> = CJHashMap::new();

    // Adding items
    let (key, value) = (48u32, String::from("Hello"));
    println!("Adding value '{value}' for key {key}");
    hashmap.set(key, value);
    let (key, value) = (32u32, String::from("Good bye"));
    println!("Adding value '{value}' for key {key}");
    hashmap.set(key, value);

    // Retrieving item
    println!("Searching for value stored in key {key}");
    match hashmap.get(key) {
        None => println!("Item not found"),
        Some(result) => println!("Key {key} contains value '{result}'"),
    }

    // Getting the current number of items on the map
    println!("Hashmap has {} item(s)", hashmap.len());

    // Removing an item
    println!("Removing value stored in key {key}");
    match hashmap.remove(key) {
        None => println!("Item not found"),
        Some(result) => println!("Removed value '{result}'"),
    }

    // New map len() after remove
    println!("Hashmap now only has {} item(s)", hashmap.len());
}
