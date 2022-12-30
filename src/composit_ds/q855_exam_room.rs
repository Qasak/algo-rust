use std::collections::{HashMap, HashSet, BTreeSet, BinaryHeap, BTreeMap, VecDeque};
struct ExamRoom {
    points: BTreeSet<i32>,
    intervals: BTreeMap<i32, BTreeMap<i32, i32>>,
    n: i32,
}

impl ExamRoom {

    fn new(n: i32) -> Self {
        Self {
            points: BTreeSet::new(),
            intervals: BTreeMap::from([(n, BTreeMap::from([(-1, n)]))]),
            n
        }
    }

    fn seat(&mut self) -> i32 {
        let mut len = *self.intervals.iter().rev().next().unwrap().0;
        if let Some(x) = self.intervals.get_mut(&len) {
            let a = x.iter().next().unwrap();
            let mut m = if *a.0 == -1 {0} else if *a.1 == self.n {self.n - 1} else {*a.0 + (*a.1 - *a.0) / 2};
            let mut sub = *a.0;
            // 偶数空位的区间x，要考虑x-1的区间
            if len & 1 == 0 && self.n != m + 1 {
                if let Some(x) = self.intervals.get_mut(&(len - 1)) {
                    let mut a = x.iter().next().unwrap();
                    let mut second = if *a.0 == -1 {0} else if *a.1 == self.n {self.n - 1} else {*a.0 + (*a.1 - *a.0) / 2};
                    if second < m {
                        len = len - 1;
                        sub = *a.0;
                        m = second;
                    }
                }
            }

            let a = sub;
            if let Some(x) = self.intervals.get_mut(&len) {
                if let Some(b) = x.remove(&a) {
                    if x.is_empty() {
                        self.intervals.remove(&len);
                    }
                    self.points.insert(m);
                    self.intervals.entry(m - a - 1).or_insert(BTreeMap::new()).insert(a, m);
                    self.intervals.entry(b - m - 1).or_insert(BTreeMap::new()).insert(m, b);
                    return m;
                }
            }
        }
        0
    }

    fn leave(&mut self, p: i32) {
        let a = self.points.range(..p).rev().next().copied().unwrap_or(-1);
        self.points.remove(&p);
        let len = p - a - 1;
        if let Some(x) = self.intervals.get_mut(&len) {
            x.remove(&a);
            if x.is_empty() {
                self.intervals.remove(&len);
            }
        }
        let b = self.points.range(p+1..).next().copied().unwrap_or(self.n);
        let len = b - p - 1;
        if let Some(x) = self.intervals.get_mut(&len) {
            x.remove(&p);
            if x.is_empty() {
                self.intervals.remove(&len);
            }
        }
        self.intervals.entry(b - a - 1).or_insert(BTreeMap::new()).insert(a, b);
    }
}