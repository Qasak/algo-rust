use std::collections::BinaryHeap;

pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let (mut q1, mut q2) = (BinaryHeap::new(), BinaryHeap::new());
    let mut k = k; let len = candidates as usize;
    let n = costs.len();
    let (mut l, mut r) = (0, n - 1);

    while l <= r && l < len && r >= n - len {
        q1.push(-costs[l] as i64);
        l += 1;
        if l <= r {q2.push(-costs[r] as i64);}
        r -= 1;
    }
    let mut ret: i64 = 0;
    while k > 0  {
        if !q1.is_empty() && !q2.is_empty() {
            if q1.peek().unwrap() == q2.peek().unwrap() {
                ret += q1.pop().unwrap();
                if l < n && r >= 0 && l <= r {q1.push(-costs[l] as i64); l += 1;}
            } else if q1.peek().unwrap() > q2.peek().unwrap() {
                ret += q1.pop().unwrap();
                if l < n && r >= 0 && l <= r {q1.push(-costs[l] as i64); l += 1;}
            } else {
                ret += q2.pop().unwrap();
                if l < n && r >= 0 && l <= r {q2.push(-costs[r] as i64); r -= 1;}
            }
        } else if !q1.is_empty() {
            ret += q1.pop().unwrap();
        } else if !q2.is_empty() {
            ret += q2.pop().unwrap();
        }
        k -= 1;
    }
    -ret
}