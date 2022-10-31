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