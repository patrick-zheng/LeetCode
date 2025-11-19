pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
         if prices.is_empty() {
            return 0;
        }

        // Use a large negative value to avoid overflow when adding.
        let mut buy1: i32 = i32::MIN / 2;
        let mut sell1: i32 = 0;
        let mut buy2: i32 = i32::MIN / 2;
        let mut sell2: i32 = 0;

        for price in prices {
            sell2 = sell2.max(buy2 + price);    // second sell
            buy2  = buy2.max(sell1 - price);    // second buy
            sell1 = sell1.max(buy1 + price);    // first sell
            buy1  = buy1.max(-price);           // first buy
        }

        sell2
    }
}
