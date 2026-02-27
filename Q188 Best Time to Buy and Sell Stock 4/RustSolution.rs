pub struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 0 || k == 0 {
            return 0;
        }

        let k = k as usize;

        // Unlimited transactions case
        if k >= n / 2 {
            let mut profit = 0;
            for i in 1..n {
                if prices[i] > prices[i - 1] {
                    profit += prices[i] - prices[i - 1];
                }
            }
            return profit;
        }

        let mut prev = vec![0i32; n];
        let mut cur = vec![0i32; n];

        for _t in 1..=k {
            let mut best = -prices[0]; // prev[0] - prices[0]
            for i in 1..n {
                cur[i] = cur[i - 1].max(prices[i] + best);
                best = best.max(prev[i] - prices[i]);
            }
            std::mem::swap(&mut prev, &mut cur);
            // cur will be overwritten next loop
        }

        prev[n - 1]
    }
}
