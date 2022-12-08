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