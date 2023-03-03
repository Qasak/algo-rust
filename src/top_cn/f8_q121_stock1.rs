use std::collections::VecDeque;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut dip = prices[0];
    for i in 1..prices.len() {
        profit = profit.max(prices[i] - dip);
        dip = dip.min(prices[i]);
    }
    profit
}
// 单调递增队列
pub fn max_profit_dq(prices: Vec<i32>) -> i32 {
    let mut chart = VecDeque::new();
    let mut profit = 0;
    // 单调递增队列
    for cur_price in prices {
        while !chart.is_empty() && cur_price < *chart.back().unwrap() {
            let dip = *chart.front().unwrap();
            let ath = chart.pop_back().unwrap();
            profit = profit.max(ath - dip);
        }
        chart.push_back(cur_price);
    }
    if !chart.is_empty() {
        let ath = *chart.back().unwrap();
        let dip = *chart.front().unwrap();
        profit = profit.max(ath - dip);
    }
    profit
}