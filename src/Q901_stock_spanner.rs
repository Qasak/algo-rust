use std::collections::VecDeque;

struct Pair{
    pub p: i32,
    pub i: usize,
}

struct StockSpanner {
    pub s: VecDeque<Pair>,
    pub cur: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        StockSpanner{
            s: VecDeque::new(),
            cur: 0
        }
    }
    // 单调递减栈
    // monotonically decreasing stack
    fn next(&mut self, price: i32) -> i32 {
        while !self.s.is_empty() && self.s.back().unwrap().p <= price {
            self.s.pop_back().unwrap();
        }
        self.cur += 1;
        let len = if self.s.is_empty() {self.cur} else {self.cur - self.s.back().unwrap().i};
        self.s.push_back(Pair{p: price, i: self.cur});
        len as i32
    }
}

#[cfg(test)]
mod test {
    use crate::Q901_stock_spanner::StockSpanner;
    /**
     ["StockSpanner","next","next","next","next","next","next","next"]
        [[],[100],[80],[60],[70],[60],[75],[85]]
     */
    #[test]
    fn t() {
        let mut s = StockSpanner::new();
        let input = vec![100, 80, 60, 70, 60, 75, 85];
        let mut ret = vec![];
        for i in input {
            ret.push(s.next(i));
        }
        println!("{:?}", ret);
        assert_eq!(ret, vec![1, 1, 1, 2, 1, 4, 6])
    }
}