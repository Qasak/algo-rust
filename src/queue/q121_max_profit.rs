use std::collections::VecDeque;

// mono queue
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut q = VecDeque::new();
    for p in prices {
        while !q.is_empty() && *q.back().unwrap() > p {
            ret = ret.max(*q.back().unwrap() - *q.front().unwrap());
            q.pop_back();
        }
        q.push_back(p);
    }
    ret = ret.max(*q.back().unwrap() - *q.front().unwrap());
    ret
}

// dp solution
pub fn max_profit_1(prices: Vec<i32>) -> i32 {
    let (mut f, mut min, n) = (vec![0; prices.len()], prices[0], prices.len());
    for i in 1..n {
        f[i] = f[i - 1].max(prices[i] - min);
        min = min.min(prices[i]);
    }
    f[n - 1]
}

// optimization dp
pub fn max_profit_2(prices: Vec<i32>) -> i32 {
    let (mut ret, mut min) = (0, i32::MAX);
    for p in prices {
        min = min.min(p);
        ret = ret.max(p - min);
    }
    ret
}

//
