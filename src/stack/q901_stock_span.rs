use std::collections::VecDeque;
struct StockSpanner {
    stk: VecDeque<(i32, i32)>,
    day: i32,
}

impl StockSpanner {
    fn new() -> StockSpanner {
        let mut stk = VecDeque::new();
        stk.push_back((-1, 0x3f3f3f3f));
        StockSpanner { stk, day: 0 }
    }

    fn next(&mut self, price: i32) -> i32 {
        while !self.stk.is_empty() && self.stk.back().unwrap().1 <= price {
            self.stk.pop_back();
        }
        let ret = self.day - self.stk.back().unwrap().0;
        self.stk.push_back((self.day, price));
        self.day += 1;
        ret
    }
}
