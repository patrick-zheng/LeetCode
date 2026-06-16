use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianFinder {
    low: BinaryHeap<i32>,
    high: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            low: BinaryHeap::new(),
            high: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.low.push(num);
        let moved = self.low.pop().unwrap();
        self.high.push(Reverse(moved));
        if self.low.len() < self.high.len() {
            let Reverse(moved) = self.high.pop().unwrap();
            self.low.push(moved);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.low.len() > self.high.len() {
            return *self.low.peek().unwrap() as f64;
        }
        let left = *self.low.peek().unwrap() as f64;
        let Reverse(right) = self.high.peek().unwrap();
        (left + *right as f64) / 2.0
    }
}
