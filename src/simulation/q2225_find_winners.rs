use std::collections::{HashMap, HashSet};
pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut cnt = vec![0; 100001];
    let mut members = HashSet::new();
    for m in matches {
        cnt[m[1] as usize] += 1;
        members.insert(m[1] as usize); members.insert(m[0] as usize);
    }
    let (mut a, mut b) = (vec![], vec![]);
    for i in members {
        if cnt[i] == 0 {
            a.push(i as i32);
        } else if cnt[i] == 1 {
            b.push(i as i32);
        }
    }
    a.sort(); b.sort();
    vec![a, b]
}

// iter soltion
pub fn find_winners_1(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let map:HashMap<i32, i32> = matches.iter().
        fold(HashMap::new(), |mut m, p| {
            *m.entry(p[0]).or_default() += 0;
            *m.entry(p[1]).or_default() += 1;
            m
        });
    let mut a: Vec<i32> = map.iter().filter(|e| *e.1 == 0).map(|e| *e.0).collect();
    let mut b: Vec<i32> = map.iter().filter(|e| *e.1 == 1).map(|e| *e.0).collect();
    a.sort(); b.sort();
    vec![a, b]
}