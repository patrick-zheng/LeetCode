use std::collections::{HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut squares = Vec::new();
        let mut j = 1;
        while j * j <= n {
            squares.push(j * j);
            j += 1;
        }

        let mut queue = VecDeque::from([n]);
        let mut visited = HashSet::from([n]);
        let mut steps = 0i32;

        while !queue.is_empty() {
            steps += 1;
            for _ in 0..queue.len() {
                let value = queue.pop_front().unwrap();
                for &sq in &squares {
                    let next = value - sq;
                    if next == 0 {
                        return steps;
                    }
                    if next > 0 && visited.insert(next) {
                        queue.push_back(next);
                    }
                }
            }
        }
        steps
    }
}
