pub struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        let mut min_val = i32::MAX;
        let mut max_val = i32::MIN;
        for &x in &nums {
            min_val = min_val.min(x);
            max_val = max_val.max(x);
        }
        if min_val == max_val {
            return 0;
        }

        // bucket_size = ceil((max - min) / (n - 1))
        let range = (max_val as i64) - (min_val as i64);
        let bucket_count = (n - 1) as usize;
        let bucket_size = ((range + (bucket_count as i64) - 1) / (bucket_count as i64)).max(1);

        let mut bucket_min = vec![i32::MAX; bucket_count];
        let mut bucket_max = vec![i32::MIN; bucket_count];
        let mut used = vec![false; bucket_count];

        for &x in &nums {
            if x == min_val || x == max_val {
                continue;
            }
            let idx = (((x as i64) - (min_val as i64)) / bucket_size) as usize;
            bucket_min[idx] = bucket_min[idx].min(x);
            bucket_max[idx] = bucket_max[idx].max(x);
            used[idx] = true;
        }

        let mut prev = min_val;
        let mut ans: i32 = 0;

        for i in 0..bucket_count {
            if !used[i] {
                continue;
            }
            ans = ans.max(bucket_min[i] - prev);
            prev = bucket_max[i];
        }

        ans = ans.max(max_val - prev);
        ans
    }
}
