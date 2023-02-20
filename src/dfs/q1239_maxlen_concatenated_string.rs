// leetcode 2022-10-24 EN

// 1. directly use string to count
// 2. convert valid string to int with bitwise op, then count
pub fn max_length(arr: Vec<String>) -> i32 {
    // use String, in each level of recursion, we have to clone it
    fn dfs(i: usize, cur: String, arr: &Vec<String>) -> i32 {
        if i == arr.len() {
            return cur.len() as i32;
        }
        let mut a = 0;
        let mut b = 0;
        // to item at index i, we can choose or not to choose
        let s = cur.clone() + arr[i].as_str();
        if check(&s) {
            a = dfs(i + 1, s, arr);
        }
        b = dfs(i + 1, cur.clone(), arr);
        b.max(a)
    }

    fn check(cur: &String) -> bool {
        let mut cnt = vec![0; 26];
        for ch in cur.bytes() {
            let idx = (ch - b'a') as usize;
            cnt[idx] += 1;
            if cnt[idx] > 1 {
                return false;
            }
        }
        true
    }
    dfs(0, "".to_string(), &arr)
}

pub fn max_length_bit(arr: Vec<String>) -> i32 {
    let mut ret: u32 = 0;
    let mut masks: Vec<u32> = vec![0];
    for s in arr {
        // convert valid strings to bit masks
        let mut mask: u32 = s.bytes().map(|c| 1 << (c - b'a')).sum();
        if mask.count_ones() == s.len() as u32 {
            masks.push(mask);
        } else {
            continue;
        }
        // combine current bit mask to existed masks, then count the combined one's len,
        // push it into our vec
        // compare with current max length
        let n = masks.len();
        for i in 0..(n - 1) {
            let m = masks[i];
            if (m & mask).count_ones() == 0 {
                masks.push(m | mask);
                ret = ret.max((m | mask).count_ones());
            }
        }
    }
    ret as i32
}
