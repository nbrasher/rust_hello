use rust_hello::LRUCache;

fn main() {
    let mut lru = LRUCache::new(5);
    lru.insert(5, 1);

    match lru.get(&5) {
        Some(v) => println!("Value is {}", *v),
        None => println!("Missing value")
    };
}
