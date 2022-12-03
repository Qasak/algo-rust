use std::collections::HashMap;
use std::cmp::Reverse;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let map:HashMap<char, u32> = s.chars().fold(HashMap::new(), |mut m, c| {
            *m.entry(c).or_default() += 1; m
        });
        let mut cs:Vec<char> = s.chars().collect();
        cs.sort_by_key(|&c| (Reverse(map[&c]), c));
        cs.into_iter().collect::<String>()
    }
}