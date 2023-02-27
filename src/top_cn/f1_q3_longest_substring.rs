use std::collections::HashMap;


pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let mut map = HashMap::new();
    let mut start = 0;
    let mut ret = 1;
    for (i, c) in s.chars().enumerate() {
        if let Some(&j) = map.get(&c) {
            start = start.max(j + 1);
        }
        map.insert(c, i);
        ret = ret.max(i - start + 1);
    }
    ret as i32
}
