use rust_hello::LRUCache;

fn main() {
    let mut lru = LRUCache::new(3);
    lru.insert("key1", 1);
    lru.insert("key2", 2);
    lru.insert("key3", 3);
    println!("Value at key1 is {}", lru.get("key1").unwrap());

    lru.insert("key4", 4);

    for key in ["key1", "key2", "key3", "key4"] {
        match lru.get(key) {
            None => println!("Key {} has been dropped", key),
            Some(value) => println!("Value at {} is {}", key, value),
        }
    }
    
}
