pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let prev = Solution::count_and_say(n - 1);
        let mut result = String::new();
        let mut chars = prev.chars().peekable();
        let mut count = 1;

        let mut current = chars.next().unwrap();
        while let Some(&next) = chars.peek() {
            if next == current {
                count += 1;
            } else {
                result.push_str(&count.to_string());
                result.push(current);
                count = 1;
                current = next;
            }
            chars.next();
        }

        result.push_str(&count.to_string());
        result.push(current);

        result
    }
}
