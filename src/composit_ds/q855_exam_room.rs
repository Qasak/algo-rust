use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

pub struct ExamRoom {
    n: i32,
    seats: BTreeSet<i32>,
    q: BinaryHeap<(i32, Reverse<i32>, i32)>,
}

impl ExamRoom {
    pub fn new(n: i32) -> Self {
        Self {
            n,
            seats: Default::default(),
            q: Default::default(),
        }
    }

    pub fn seat(&mut self) -> i32 {
        if self.seats.is_empty() {
            self.seats.insert(0);
            return 0;
        }
        let first = *self.seats.iter().next().unwrap();
        let last = *self.seats.iter().last().unwrap();
        let left = first;
        let right = self.n - 1 - last;
        while let Some(&(d, Reverse(l), r)) = self.q.peek() {
            if
            // !self.seats.contains(&l)||
            !self.seats.contains(&r) ||
                *self.seats.range(l + 1..).next().unwrap() != r
            {
                self.q.pop();
                continue;
            }
            if d < right || d <= left {
                break;
            }
            self.q.pop();
            self.q.push((d / 2, Reverse(l), l + d));
            self.q.push(((r - (l + d)) / 2, Reverse(l + d), r));
            self.seats.insert(l + d);
            return l + d;
        }
        if right > left {
            self.q.push(((self.n - 1 - last) / 2, Reverse(last), self.n - 1));
            self.seats.insert(self.n - 1);
            self.n - 1
        } else {
            self.q.push((first / 2, Reverse(0), first));
            self.seats.insert(0);
            0
        }
    }

    pub fn leave(&mut self, p: i32) {
        if p != *self.seats.iter().next().unwrap() && p != *self.seats.iter().last().unwrap() {
            let l = *self.seats.range(..p).last().unwrap();
            let r = *self.seats.range(p + 1..).next().unwrap();
            self.q.push(((r - l) / 2, Reverse(l), r));
        }
        self.seats.remove(&p);
    }
}