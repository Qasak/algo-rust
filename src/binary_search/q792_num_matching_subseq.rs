use std::collections::HashMap;

// pre-process + binary_search
pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let mut map: HashMap<char, Vec<usize>> = HashMap::new();
    let mut cs = s.chars().collect::<Vec<char>>();
    let n = cs.len();
    let mut cnt = 0;
    for i in 0..n {
        if let Some(list) = map.get_mut(&cs[i]) {
            list.push(i);
        } else {
            map.insert(cs[i], vec![i]);
        }
    }
    for w in words {
        let mut idx = -1;
        let mut find = true;
        for c in w.chars() {
            if let Some(list) = map.get(&c) {
                let m = list.len();
                let (mut l, mut r) = (0, m);
                while l < r {
                    let m = l + (r - l) / 2;
                    if list[m] as i32 <= idx {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                if l == m || list[l] as i32 <= idx {
                    find = false;
                    break;
                } else {
                    idx = list[l] as i32;
                }
            } else {
                find = false;
                break;
            }
        }
        if find {
            cnt += 1;
        }
    }
    cnt
}
