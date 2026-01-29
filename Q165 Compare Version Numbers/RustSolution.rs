pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<i32> = version1.split('.').map(|s| s.parse::<i32>().unwrap()).collect();
        let v2: Vec<i32> = version2.split('.').map(|s| s.parse::<i32>().unwrap()).collect();

        let n = std::cmp::max(v1.len(), v2.len());

        for i in 0..n {
            let a = if i < v1.len() { v1[i] } else { 0 };
            let b = if i < v2.len() { v2[i] } else { 0 };

            if a < b { return -1; }
            if a > b { return 1; }
        }

        0
    }
}
