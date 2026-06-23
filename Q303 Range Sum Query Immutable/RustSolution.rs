pub struct NumArray {
    prefix: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![0];
        for num in nums {
            prefix.push(prefix.last().unwrap() + num);
        }
        Self { prefix }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[(right + 1) as usize] - self.prefix[left as usize]
    }
}
