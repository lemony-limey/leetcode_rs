use std::collections::HashMap;

struct LRUCache
{
    // The key maps to the index in the vec-based linked list.
    cache_map:   HashMap<Key, CacheIndex>,
    cache:       Vec<CacheEntry>,
    head_newest: CacheIndex,
    tail_oldest: CacheIndex,
}

type Key        = i32;
type Value      = i32;
type CacheIndex = usize;

struct CacheEntry
{
    next:        usize,
    prev:        usize,
    value:       Value,
}

impl LRUCache
{
    fn new(capacity: i32) -> Self
    {
        // Ensure the capacity is positive.
        assert!(capacity > 0);

        let capacity = capacity as usize;

        Self
        {
            cache_map:   HashMap::with_capacity(capacity),
            cache:       Vec::with_capacity(capacity),
            head_newest: 0,
            tail_oldest: 0,
        }
    }

    fn get(&self, key: Key) -> Value
    {
        // Get the index from the HashMap, returning -1 if it is not present.
        let Some(index) = self.cache_map.get(&key) else {
            return -1;
        };

        self.cache[*index].value
    }

    fn put(&mut self, key: Key, value: Value)
    {
        // Initially, everything is just added to the cache.
        if self.cache.len() >= CAPACITY
        {

        }
        // Compulsory misses.
        else
        {
            let cache_entry: CacheEntry;
            // First node is placed in slot 0.
            if self.cache.is_empty()
            {
                self.head_newest = 0;
                self.head_oldest = 0;
                self.cache.push(CacheEntry {
                    next: 0,
                    prev: 0,
                    value,
                });
            }
            else
            {
                // Add node
            }
        
            self.cache.push(cache_entry);
        }
    }
}

// The LRUCache object will be instantiated and called as such:
// let obj = LRUCache::new(capacity);
// let ret_1: i32 = obj.get(key);
// obj.put(key, value);
