use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut cash: i32 = 0;
        let mut hold: i32 = -prices[0];
        
        for price in prices.iter().skip(1) {
            cash = cmp::max(cash, hold + *price - fee);
            hold = cmp::max(hold, cash - *price);
        }
        
        cash
    }
}
