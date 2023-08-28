# rust-map
An implementation of a hash map in Rust. For learning purposes only, should not be used in production (for now).

The hash map uses the SipHash algorithm (https://en.wikipedia.org/wiki/SipHash) for hashing the keys.


## Usage

### Initialization
Create a new hash map with `let mut hashmap: CJHashMap<K, V> = CJHashMap::new();`, where `K` and `V` are the types of the keys and the values.

Ex:
```
let mut hashmap: CJHashMap<u32, String> = CJHashMap::new();
```


### Adding items
New items can be added with the `.set()` method.

Ex:
```
hashmap.set(key, value);
```


### Retrieving values
A value for a stored key can be retrieved with the `.get()` method.

Ex:
```
let value = hashmap.get(key);
```


### Removing items
Items can be removed with the `.remove()` method

Ex:
```
let removed = hashmap.remove(key);
if removed.is_some() {
  println!("Value of key {key} was removed");
}
```

The `remove()` method returns `Option<V>`. I.e, it returns `Some(value)` (where "value" is the value of the removed key) if the key was found and removed and `None` if it wasn't.

### Number of items on the map
The length of the map - the number of (key, value) pairs it contains can be retrieved with the `.len()` method.

Ex:
```
if hashmap.len() == 0 {
  println!("Map is empty!");
}
```
