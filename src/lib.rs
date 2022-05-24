use std::{ collections::HashMap, vec::Vec };

struct Node {
    key: usize,
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
        cache.push(Node{ key: 0, value: 0, next: Some(1), prev: None });
        cache.push(Node{ key: 0, value: 0, next: None, prev: Some(0) });

        LRUCache { capacity, size: 0, map, cache }
    }
    pub fn insert(&mut self, key: usize, value: usize) -> () {
        if let Some(ix) = self.map.get(&key) {
            self.cache[*ix].value = value;
        } else {
            if self.size >= self.capacity {
                // Update pointers to remove least-recently used node
                let to_update = self.cache[1].prev.unwrap();

                // Remove old key from map
                let old_key = self.cache[to_update].key;
                self.map.remove(&old_key);

                let prev_ix = self.cache[to_update].prev.unwrap();
                let next_ix = self.cache[to_update].next.unwrap();

                self.cache[prev_ix].next = Some(next_ix);
                self.cache[next_ix].prev = Some(prev_ix);
                
                // Insert new node at head
                let head_next = self.cache[0].next.unwrap();
                self.cache[to_update] = Node{ key, value, next: Some(head_next), prev: Some(0) };
                self.cache[0].next = Some(to_update);
                self.cache[head_next].prev = Some(to_update);
                self.map.insert(key, to_update);
            } else {
                // Create new node, insert to cache
                let head_next = self.cache[0].next.unwrap();
                self.cache.push(Node{ key, value, next: Some(head_next), prev: Some(0) });

                // Update pointers, p-head.next and p-head.next.prev -> new node
                let new_ix = self.size + 2;
                self.cache[0].next = Some(new_ix);
                self.cache[head_next].prev = Some(new_ix);

                self.map.insert(key, new_ix);
                self.size += 1;
            }
        }
    }
    pub fn get(&mut self, key: usize) -> Option<usize> {
        if let Some(ix) = self.map.get(&key) {
            let head_next = self.cache[0].next.unwrap();
            self.cache[0].next = Some(*ix);
            self.cache[head_next].prev = Some(*ix);

            Some(self.cache[*ix].value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_size() {
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

    #[test]
    fn test_lru_drop() {
        let mut lru = LRUCache::new(3);
        lru.insert(1, 1);
        lru.insert(2, 5);
        lru.insert(3, 3);
        lru.insert(4, 4);

        // First inserted key is dropped
        assert_eq!(lru.get(1), None);

        // Other keys are correct
        assert_eq!(lru.get(2), Some(5));
        assert_eq!(lru.get(3), Some(3));
        assert_eq!(lru.get(4), Some(4));

        lru.insert(5, 6);

        // Last access key is dropped
        assert_eq!(lru.get(2), None);
        assert_eq!(lru.get(5), Some(6));
    }
}