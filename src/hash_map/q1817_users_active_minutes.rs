use std::collections::{HashMap, HashSet};

pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut ret = vec![0; k as usize];
    let mut cnt = HashMap::new();
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for log in logs.iter() {
        let (id, time) = (log[0], log[1]);
        let set = map.entry(id).or_default();
        if !set.contains(&time) {
            *cnt.entry(id).or_insert(0) += 1;
            set.insert(time);
        }
    }
    for (_, v) in cnt {
        ret[v - 1] += 1;
    }
    ret
}