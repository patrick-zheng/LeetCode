pub struct NumArray {
    nums: Vec<i32>,
    tree: Vec<i32>,
    n: usize,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut fenwick = Self {
            nums: nums.clone(),
            tree: vec![0; n + 1],
            n,
        };
        for i in 0..n {
            fenwick.add(i + 1, nums[i]);
        }
        fenwick
    }

    fn add(&mut self, mut i: usize, delta: i32) {
        while i <= self.n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix(&self, mut i: usize) -> i32 {
        let mut total = 0;
        while i > 0 {
            total += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        total
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let delta = val - self.nums[index];
        self.nums[index] = val;
        self.add(index + 1, delta);
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix((right + 1) as usize) - self.prefix(left as usize)
    }
}
