use std::{ collections::HashMap, vec::Vec };

#[derive(Copy, Clone)]
enum Ptr {
    Head,
    Tail,
    Index(usize),
}

struct Node {
    key: usize,
    value: usize,
    prev: Ptr,
    next: Ptr,
}

pub struct LRUCache {
    capacity: usize,
    size: usize,
    head: Ptr,
    tail: Ptr,
    map: HashMap<usize, usize>,
    cache: Vec<Node>
}

impl LRUCache {
    pub fn new(capacity: usize) -> LRUCache {
        LRUCache { 
            capacity,
            size: 0,
            head: Ptr::Tail,
            tail: Ptr::Head, 
            map: HashMap::new(),
            cache: Vec::with_capacity(capacity),
        }
    }
    pub fn insert(&mut self, key: usize, value: usize) -> () {
        if let Some(ix) = self.map.get(&key) {
            self.cache[*ix].value = value;
        } else {
            if self.size >= self.capacity {
                // Update pointers to remove least-recently used node
                if let Ptr::Index(to_update) = self.tail {

                    // Remove old key from map
                    let old_key = self.cache[to_update].key;
                    self.map.remove(&old_key);

                    // TODO - figure out how to better update these pointers
                    match self.cache[to_update].prev {
                        Ptr::Head => self.head = self.cache[to_update].next,
                        Ptr::Tail => (),
                        Ptr::Index(ix) => self.cache[ix].next = self.cache[to_update].next,
                    };
                    match self.cache[to_update].next {
                        Ptr::Head => (),
                        Ptr::Tail => self.tail = self.cache[to_update].prev,
                        Ptr::Index(ix) => self.cache[ix].prev = self.cache[to_update].prev,
                    };
                    
                    // Insert new node at head
                    self.cache[to_update] = Node{ key, value, next: self.head, prev: Ptr::Head };
                    self.cache[0].next = Ptr::Index(to_update);
                    if let Ptr::Index(head_next) = self.head {
                        self.cache[head_next].prev = Ptr::Index(to_update);
                    }
                    self.map.insert(key, to_update);
                }
            } else {
                // Create new node, insert to cache
                self.cache.push(Node{ key, value, next: self.head, prev: Ptr::Head });

                // Update pointers, p-head.next and p-head.next.prev -> new node
                let new_ix = self.size;
                
                // If list was non-empty, update previous head, else update tail
                if let Ptr::Index(head_next) = self.head {
                    self.cache[head_next].prev = Ptr::Index(new_ix);
                } else {
                    self.tail = Ptr::Index(new_ix);
                }
                
                self.head = Ptr::Index(new_ix);

                self.map.insert(key, new_ix);
                self.size += 1;
            }
        }
    }
    pub fn get(&mut self, key: usize) -> Option<usize> {
        if let Some(ix) = self.map.get(&key) {
            if let Ptr::Index(head_next) = self.head {
                self.head = Ptr::Index(*ix);
                self.cache[head_next].prev = Ptr::Index(*ix);
                Some(self.cache[*ix].value)
            } else {
                None
            }
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