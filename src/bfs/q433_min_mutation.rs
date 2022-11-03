use std::collections::{HashSet, VecDeque};

pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
    fn compare(s1: &String, s2: &String) -> bool {
        let cnt = s1.chars().zip(s2.chars()).fold(0, |cnt, (ch1, ch2)| if ch1 == ch2 { cnt } else { cnt + 1 });
        if cnt > 1 { false } else { true }
    }
    fn dfs<'a>(ans: &mut i32, cnt: i32, cur: &String, end: &String, bank: &'a Vec<String>, vis: &mut HashSet<&'a String>) {
        if cur == end {
            *ans = (*ans).min(cnt);
            return;
        }
        for s in bank.iter() {
            if compare(s, cur) && !vis.contains(s) {
                vis.insert(s);
                dfs(ans, cnt + 1, s, end, bank, vis);
                vis.remove(s);
            }
        }
    }
    let ans = &mut i32::MAX;
    dfs(ans, 0, &start, &end, &bank, &mut HashSet::new());
    if *ans == i32::MAX {-1} else {*ans}
}
pub fn min_mutation_bfs(start: String, end: String, bank: Vec<String>) -> i32 {
    fn compare(s1: &String, s2: &String) -> bool {
        let cnt = s1.chars().zip(s2.chars()).fold(0, |cnt, (ch1, ch2)| if ch1 == ch2 { cnt } else { cnt + 1 });
        if cnt > 1 { false } else { true }
    }
    let mut q = VecDeque::new();
    let mut vis = HashSet::new();
    q.push_back((&start, 0));
    vis.insert(&start);
    while let Some((s, step)) = q.pop_front() {
        if s == &end {
            return step;
        }
        for ss in bank.iter() {
            if compare(ss, s) && !vis.contains(ss) {
                q.push_back((ss, step + 1));
                vis.insert(ss);
            }
        }
    }
    -1
}
