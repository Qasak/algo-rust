use std::collections::BTreeSet;

struct ExamRoom {
    cap: i32,
    seats: BTreeSet<i32>,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            cap: n,
            seats: BTreeSet::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        if self.seats.is_empty() {
            self.seats.insert(0);
            return 0;
        }
        let (mut mid, mut l, mut len) = (0, 0, 0);
        for (i, &r) in self.seats.iter().enumerate() {
            if i == 0 && r != 0 {
                len = r;
            } else if (r - l) / 2 > len{
                len =  (r - l) / 2;
                mid = (l + r) / 2;
            }
            l = r;
        }
        if self.cap - 1 - l > len {
            mid = self.cap - 1;
        }
        self.seats.insert(mid);
        mid
    }

    fn leave(&mut self, p: i32) {
        self.seats.remove(&p);
    }
}