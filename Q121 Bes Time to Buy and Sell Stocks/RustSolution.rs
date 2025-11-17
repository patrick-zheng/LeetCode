pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut min_price = prices[0];
        let mut max_profit = 0;

        for &price in prices.iter().skip(1) {
            // Profit if we sell today
            let profit = price - min_price;
            if profit > max_profit {
                max_profit = profit;
            }

            // Update minimum price seen so far
            if price < min_price {
                min_price = price;
            }
        }

        max_profit
    }
}
