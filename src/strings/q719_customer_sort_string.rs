use std::collections::HashMap;

// customer sort
pub fn custom_sort_string(order: String, s: String) -> String {
    let mut map = HashMap::new();
    for (i, c) in order.chars().enumerate() {
        map.insert(c, i);
    }
    let mut cs = s.chars().collect::<Vec<char>>();
    cs.sort_unstable_by_key(|item| if map.contains_key(item) {*map.get(item).unwrap()} else {0});
    cs.into_iter().collect::<String>()
}

// counting words
pub fn custom_sort_string_count(order: String, s: String) -> String {
    let mut cnt = vec![0; 26];
    let mut ret = vec![];
    for c in s.bytes() {cnt[(c - b'a') as usize] += 1;}
    for c in order.bytes() {
        while cnt[(c - b'a') as usize] > 0 {ret.push(c as char); cnt[(c - b'a') as usize] -= 1;}
    }
    for i in 0..26 {
        while cnt[i] > 0 {ret.push((i as u8 + b'a') as char); cnt[i] -= 1;}
    }
    ret.into_iter().collect()
}