// count connected components: m
// stones: n
// n - m

use std::collections::{HashMap, HashSet, VecDeque};
// BFS to find connected component
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let (n, mut c) = (stones.len(), 0);
    let rows: HashMap<i32,Vec<usize>> = stones.iter().enumerate()
        .fold(HashMap::new(), |mut m,(s,v)| { m.entry(v[0]).or_default().push(s); m });
    let cols: HashMap<i32,Vec<usize>> = stones.iter().enumerate()
        .fold(HashMap::new(), |mut m,(s,v)| { m.entry(v[1]).or_default().push(s); m });

    let mut seen = HashSet::new();
    for s in 0..n {
        if !seen.contains(&s) {
            let mut dq = VecDeque::from(vec![s]);
            while let Some(s) = dq.pop_front() {
                if seen.contains(&s) { continue; }
                seen.insert(s);
                let (r, c) = (stones[s][0], stones[s][1]);
                for ss in rows[&r].iter().chain(cols[&c].iter()) { dq.push_back(*ss); }
            }
            c += 1;
        }
    }
    return (n - c) as i32;
}