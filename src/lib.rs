use std::{ collections::HashMap, vec::Vec, hash::Hash };

#[derive(Copy, Clone)]
enum Ptr {
    Head,
    Tail,
    Index(usize),
}

struct Node<'a, K, V> {
    key: &'a K,
    value: &'a V,
    prev: Ptr,
    next: Ptr,
}

pub struct LRUCache<'a, K, V> {
    capacity: usize,
    size: usize,
    head: Ptr,
    tail: Ptr,
    map: HashMap<&'a K, usize>,
    cache: Vec<Node<'a, K, V>>
}

impl <'a, K, V> LRUCache<'a, K, V> 
where K: Eq + Hash
{
    pub fn new(capacity: usize) -> LRUCache<'a, K, V> {
        LRUCache { 
            capacity,
            size: 0,
            head: Ptr::Tail,
            tail: Ptr::Head, 
            map: HashMap::<&'a K, usize>::new(),
            cache: Vec::<Node<K, V>>::with_capacity(capacity),
        }
    }
    fn move_to_head(&mut self, to_move: usize) -> () {
        match self.cache[to_move].prev {
            Ptr::Head => (),
            Ptr::Tail => (),
            Ptr::Index(ix) => self.cache[ix].next = self.cache[to_move].next,
        };
        match self.cache[to_move].next {
            Ptr::Head => (),
            Ptr::Tail => self.tail = self.cache[to_move].prev,
            Ptr::Index(ix) => self.cache[ix].prev = self.cache[to_move].prev,
        };
        match self.head {
            Ptr::Head => (),
            Ptr::Tail => self.tail = Ptr::Index(to_move),
            Ptr::Index(ix) => self.cache[ix].prev = Ptr::Index(to_move),
        };
        self.cache[to_move].next = self.head;
        self.head = Ptr::Index(to_move);
        self.cache[to_move].prev = Ptr::Head;
    }
    pub fn insert(&mut self, key: &'a K, value: &'a V) -> () {
        if let Some(ix) = self.map.get(&key) {
            let ins_ix = *ix;
            self.move_to_head(ins_ix);
            self.cache[ins_ix].value = value;
        } else {
            if self.size >= self.capacity {

                // self.tail should be a Ptr::Index unless the cache is empty
                if let Ptr::Index(to_update) = self.tail {

                    // Remove old key from map
                    self.map.remove(&self.cache[to_update].key);
                    
                    // Insert new node at head
                    self.move_to_head(to_update);
                    self.cache[to_update].key = key;
                    self.cache[to_update].value = value;
                    self.map.insert(key, to_update);
                }
            } else {
                let to_update = self.size;

                self.cache.push(Node{ key, value, next: self.head, prev: Ptr::Head });
                self.move_to_head(to_update);
                self.map.insert(key, to_update);

                self.size += 1;
            }
        }
    }
    pub fn get(&mut self, key: &'a K) -> Option<&'a V> {
        match self.map.get(&key) {
            Some(ix) => {
                let ret_ix = *ix;
                self.move_to_head(ret_ix);
                Some(self.cache[ret_ix].value)
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_size() {
        let lru = LRUCache::<u32, u32>::new(5);

        assert_eq!(lru.capacity, 5);
        assert_eq!(lru.size, 0);
    }

    #[test]
    fn test_lru_put_get() {
        let mut lru = LRUCache::new(5);
        lru.insert(&5, &1);
        lru.insert(&5, &5);
        lru.insert(&3, &3);

        assert_eq!(lru.get(&5), Some(&5));
        assert_eq!(lru.get(&3), Some(&3));
        assert_eq!(lru.get(&1), None);
    }

    #[test]
    fn test_ins_drop() {
        let mut lru = LRUCache::new(3);
        lru.insert(&1, &1);
        lru.insert(&2, &5);
        lru.insert(&3, &3);
        lru.insert(&4, &4);

        // First inserted key is dropped
        assert_eq!(lru.get(&1), None);

        // Other keys are correct
        assert_eq!(lru.get(&2), Some(&5));
        assert_eq!(lru.get(&3), Some(&3));
        assert_eq!(lru.get(&4), Some(&4));

        lru.insert(&5, &6);

        // Last access key is dropped
        assert_eq!(lru.get(&2), None);
        assert_eq!(lru.get(&5), Some(&6));
    }
    
    #[test]
    fn test_get_drop() {
        let mut lru = LRUCache::new(3);
        lru.insert(&"key1", &1);
        lru.insert(&"key2", &2);
        lru.insert(&"key3", &3);

        assert_eq!(lru.get(&"key1"), Some(&1));

        // Should drop "key2" since "key1" was just read above
        lru.insert(&"key4", &4);

        assert_eq!(lru.get(&"key1"), Some(&1));
        assert_eq!(lru.get(&"key2"), None);
        assert_eq!(lru.get(&"key3"), Some(&3));
        assert_eq!(lru.get(&"key4"), Some(&4));
    }
}