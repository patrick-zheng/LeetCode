pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        // Sort by start
        intervals.sort_by_key(|x| x[0]);

        let mut merged = Vec::new();
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];

        for interval in intervals.into_iter().skip(1) {
            let (curr_start, curr_end) = (interval[0], interval[1]);

            if curr_start <= end {
                // Overlap → extend
                end = end.max(curr_end);
            } else {
                merged.push(vec![start, end]);
                start = curr_start;
                end = curr_end;
            }
        }

        merged.push(vec![start, end]);
        merged
    }
}
