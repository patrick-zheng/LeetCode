pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut a: Vec<String> = nums.into_iter().map(|x| x.to_string()).collect();

        a.sort_by(|s1, s2| {
            let ab = format!("{}{}", s1, s2);
            let ba = format!("{}{}", s2, s1);
            ba.cmp(&ab) // sort descending: if ab > ba, s1 should come first
        });

        if !a.is_empty() && a[0] == "0" {
            return "0".to_string();
        }

        a.concat()
    }
}
