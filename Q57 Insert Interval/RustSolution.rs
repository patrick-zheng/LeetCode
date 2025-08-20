pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut i = 0usize;
        let n = intervals.len();
        let (mut s, mut e) = (new_interval[0], new_interval[1]);

        // 1) Add intervals that end before new_interval starts
        while i < n && intervals[i][1] < s {
            res.push(intervals[i].clone());
            i += 1;
        }

        // 2) Merge all overlapping intervals
        while i < n && intervals[i][0] <= e {
            s = s.min(intervals[i][0]);
            e = e.max(intervals[i][1]);
            i += 1;
        }
        res.push(vec![s, e]);

        // 3) Add the remaining intervals
        while i < n {
            res.push(intervals[i].clone());
            i += 1;
        }

        res
    }
}
