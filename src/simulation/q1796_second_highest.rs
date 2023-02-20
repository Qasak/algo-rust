use std::collections::BTreeSet;

pub fn second_highest(s: String) -> i32 {
    let set: BTreeSet<i32> = s
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .map(|c| (c - b'0') as i32)
        .collect();
    let n = set.len();
    if n <= 1 {
        -1
    } else {
        set.into_iter().nth(n - 2).unwrap()
    }
}
