use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut set = HashSet::new();
    let cs = s.chars().collect::<Vec<char>>();
    let mut l = 0;
    let n = s.len();
    let mut ret = 0;
    for i in 0..n {
        if !set.contains(&cs[i]) {
            set.insert(cs[i]);
        } else {
            while l < i && cs[l] != cs[i] {
                set.remove(&cs[l]);
                l += 1;
            }
            l += 1;
        }
        ret = ret.max(set.len())
    }
    ret as i32
}

pub fn length_of_longest_substring_1(s: String) -> i32 {
    let n = s.len();
    let mut left: Vec<i32> = vec![-1; 128];
    let s = s.chars().collect::<Vec<char>>();
    let mut l = -1;
    let mut ret = 0;
    for r in 0..n {
        // maintain left bound
        l = l.max(left[s[r] as usize]);
        // cur len
        ret = ret.max(r as i32 - l);
        // update current item's last index
        left[s[r] as usize] = r as i32;
    }
    ret as i32
}