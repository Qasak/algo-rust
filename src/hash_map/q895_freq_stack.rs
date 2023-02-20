use std::collections::{HashMap, VecDeque};
struct FreqStack {
    // val: freq
    map: HashMap<i32, usize>,
    // stacks
    stacks: Vec<VecDeque<i32>>,
}

impl FreqStack {
    fn new() -> Self {
        FreqStack {
            map: HashMap::new(),
            stacks: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        *self.map.entry(val).or_default() += 1;
        let freq = self.map[&val];
        if freq > self.stacks.len() {
            self.stacks.push(VecDeque::new());
        }
        self.stacks[freq - 1].push_back(val);
    }

    fn pop(&mut self) -> i32 {
        let val = self.stacks.last_mut().unwrap().pop_back().unwrap();
        *self.map.entry(val).or_default() -= 1;
        if self.stacks.last().unwrap().is_empty() {
            self.stacks.pop();
        }
        val
    }
}
