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
        let mut map = HashMap::new();
        let mut cache = Vec::with_capacity(capacity + 2);
        
        // Pseudo-head and tail for LL implementation
        let mut p_head = Node { value: 0, next: Some(1), prev: None };
        let mut p_tail = Node { value: 0, next: None, prev: Some(0) };
        cache.push(p_head);
        cache.push(p_tail);

        LRUCache { capacity, size: 0, map, cache }
    }
    pub fn insert(&mut self, key: usize, value: usize) {
        self.map.insert(key, value);
    }
    pub fn get(&self, key: &usize) -> Option<&usize> {
        self.map.get(key)
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

        assert_eq!(lru.get(&5), Some(&5));
        assert_eq!(lru.get(&3), Some(&3));
        assert_eq!(lru.get(&1), None);
    }
}