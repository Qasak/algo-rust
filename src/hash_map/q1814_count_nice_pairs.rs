use std::collections::HashMap;

pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    fn rev(n: i32) -> i32 { n.to_string().chars().rev().collect::<String>().parse::<i32>().unwrap() }
    let (n, mut ret, mo, mut map) = (nums.len(), 0, 1e9 as i32 + 7, HashMap::new());
    for i in nums {
        let k = i - rev(i);
        let entry = map.entry(k).or_insert(0);
        ret += *entry; ret %= mo;
        *entry += 1;
    }
    ret
}