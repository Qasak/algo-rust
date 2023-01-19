use std::collections::{BTreeMap, VecDeque};
#[derive(Debug)]
struct MultiSet(BTreeMap<i32, i32>);
// MultiSet: BTreeSet但是可以有重复值
impl MultiSet {
    pub fn new() -> Self {
        MultiSet(BTreeMap::new())
    }
    pub fn contains(&self, x: i32) -> bool {
        self.0.contains_key(&x)
    }
    pub fn insert(&mut self, x: i32) {
        // self.0.entry(x).and_modify(|x| *x += 1).or_insert(1);
        *self.0.entry(x).or_default() += 1;
    }
    pub fn remove(&mut self, x: i32) {
        *self.0.entry(x).or_default() -= 1;
        if self.0[&x] == 0 {
            self.0.remove(&x);
        }
    }
    pub fn first(&self) -> i32 {
        *self.0.keys().next().unwrap()
    }
    pub fn pop_first(&mut self) -> i32 {
        let first = self.first();
        self.remove(first);
        return first;
    }
    pub fn last(&self) -> i32 {
        *self.0.keys().rev().next().unwrap()
    }
    pub fn pop_last(&mut self) -> i32 {
        let last = self.last();
        self.remove(last);
        return last;
    }
}
struct MKAverage {
    m: i32,
    k: i32,
    dq: VecDeque<i32>,
    l: MultiSet,
    r: MultiSet,
    mid: MultiSet,
    sum: i64,
}


impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        MKAverage {
            m,
            k,
            dq: VecDeque::new(),
            l: MultiSet::new(),
            r: MultiSet::new(),
            mid: MultiSet::new(),
            sum: 0,
        }
    }

    fn add_element(&mut self, cur: i32) {

        // cur加入dq
        self.dq.push_back(cur);
        if self.dq.len() <= self.m as usize {
            self.mid.insert(cur);
            self.sum += cur as i64;
            // 初次划分l, mid, r
            if self.dq.len() == self.m as usize {
                for _ in 0..self.k {
                    //safe m > 2 * k
                    // 拿走mid的前后k个值
                    // 维护mid的总和sum
                    // 将拿走的值分别放到l, r
                    let first = self.mid.first();
                    self.mid.remove(first);
                    self.sum -= first as i64;
                    self.l.insert(first);

                    let last = self.mid.last();
                    self.mid.remove(last);
                    self.sum -= last as i64;
                    self.r.insert(last);
                }
            }
        } else {
            //len == m+1
            // 将cur插入合适的set
            // 让左右两个set始终保持k个数，中间多一个数
            let l_last = self.l.last();
            let r_first = self.r.first();
            if cur < l_last {
                self.l.insert(cur);
                self.l.remove(l_last);
                self.mid.insert(l_last);
                self.sum += l_last as i64;
            } else if cur > r_first {
                self.r.insert(cur);
                self.r.remove(r_first);
                self.mid.insert(r_first);
                self.sum += r_first as i64;
            } else {
                self.mid.insert(cur);
                self.sum += cur as i64;
            }

            // 维护dq大小为m, 删除最先(最左)插入的数temp
            let temp = self.dq.pop_front().unwrap();
            // 维护三个set, 找到删除的数temp的位置并删除
            // 删除的位置在左右，那么从中间移一个数过去
            if self.l.contains(temp) {
                self.l.remove(temp);
                let first = self.mid.first();
                self.mid.remove(first);
                self.sum -= first as i64;
                self.l.insert(first);
            } else if self.r.contains(temp) {
                self.r.remove(temp);
                let last = self.mid.last();
                self.mid.remove(last);
                self.sum -= last as i64;
                self.r.insert(last);
            } else {
                self.mid.remove(temp);
                self.sum -= temp as i64;
            }
        }

    }

    fn calculate_mk_average(&self) -> i32 {
        if self.dq.len() < self.m as usize {
            return -1;
        }
        return (self.sum / (self.m - self.k * 2) as i64) as i32;
    }
}
