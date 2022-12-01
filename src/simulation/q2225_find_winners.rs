use std::collections::HashSet;
impl Solution {
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
}