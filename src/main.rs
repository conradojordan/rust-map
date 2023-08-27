mod hashmap;

use hashmap::CustomHashMap;

fn main() {
    let mut hm = CustomHashMap::new();

    let key: i32 = 3;
    let value: i32 = 8;

    hm.add(key, value);

    // for i in "Hello".as_bytes().chunks(2) {
    //     print!("chunk = {:?} ", i);
    // }
    // println!();
    // println!("\"Hello\" as bytes {:?} ", "Hello".as_bytes());

    match hm.get(key) {
        None => println!("Element not found"),
        Some(result) => println!("Element {} is {}", key, result),
    }
}
