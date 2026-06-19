pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails: Vec<i32> = Vec::new();
        for num in nums {
            let pos = tails.partition_point(|&x| x < num);
            if pos == tails.len() {
                tails.push(num);
            } else {
                tails[pos] = num;
            }
        }
        tails.len() as i32
    }
}
