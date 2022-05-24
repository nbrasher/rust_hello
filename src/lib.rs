use std::{ collections::HashMap, vec::Vec };

struct Node {
    value: usize,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LRUCache {
    capacity: usize,
    size: usize,
    map: HashMap<usize, usize>,
    cache: Vec<Node>
}

impl LRUCache {
    pub fn new(capacity: usize) -> LRUCache {
        let map = HashMap::new();
        let mut cache = Vec::with_capacity(capacity + 2);
        
        // Pseudo-head and tail for LL implementation, head will be at index 0, tail at 1
        cache.push(Node{ value: 0, next: Some(1), prev: None });
        cache.push(Node{ value: 0, next: None, prev: Some(0) });

        LRUCache { capacity, size: 0, map, cache }
    }
    pub fn insert(&mut self, key: usize, value: usize) {
        if self.map.contains_key(&key) {
            let ix = self.map.get(&key).unwrap();
            self.cache[*ix].value = value;
        } else {
            // TODO - handle at capacity case

            // Create new node, insert to cache
            let head_next = self.cache[0].next;
            self.cache.push(Node{ value, next: head_next, prev: Some(0) });

            // Update pointers, p-head.next and p-head.next.prev -> new node
            let head_next = self.cache[0].next.unwrap();
            self.cache[0].next = Some(self.size + 2);
            self.cache[head_next].prev = Some(self.size + 2);

            self.map.insert(key, self.size + 2);
            self.size += 1;
        }
    }
    pub fn get(&self, key: usize) -> Option<usize> {
        match self.map.get(&key) {
            Some(ix) => Some(self.cache[*ix].value),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_size () {
        let lru = LRUCache::new(5);

        assert_eq!(lru.capacity, 5);
        assert_eq!(lru.size, 0);
    }

    #[test]
    fn test_lru_put_get() {
        let mut lru = LRUCache::new(5);
        lru.insert(5, 1);
        lru.insert(5, 5);
        lru.insert(3, 3);

        assert_eq!(lru.get(5), Some(5));
        assert_eq!(lru.get(3), Some(3));
        assert_eq!(lru.get(1), None);
    }
}