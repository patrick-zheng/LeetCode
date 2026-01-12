use std::collections::{HashMap, VecDeque};

pub struct Solution;

impl Solution {
    struct LRUCache {
        capacity: usize,
        map: HashMap<i32, i32>,
        order: VecDeque<i32>,
    }

    impl LRUCache {
        fn new(capacity: i32) -> Self {
            Self {
                capacity: capacity as usize,
                map: HashMap::new(),
                order: VecDeque::new(),
            }
        }

        fn get(&mut self, key: i32) -> i32 {
            if !self.map.contains_key(&key) {
                return -1;
            }
            self.order.retain(|&k| k != key);
            self.order.push_front(key);
            *self.map.get(&key).unwrap()
        }

        fn put(&mut self, key: i32, value: i32) {
            if self.map.contains_key(&key) {
                self.order.retain(|&k| k != key);
            }

            self.order.push_front(key);
            self.map.insert(key, value);

            if self.map.len() > self.capacity {
                if let Some(lru) = self.order.pop_back() {
                    self.map.remove(&lru);
                }
            }
        }
    }
}
