use std::collections::HashSet;

// simulation
pub fn reverse_vowels(s: String) -> String {
    let mut vow = vec![];
    let mut idx = vec![];
    let vowels = "aeiouAEIOU";
    for (i, ch) in s.chars().enumerate() {
        if vowels.contains(ch) {
            idx.push(i);
            vow.push(ch);
        }
    }
    let mut ret = vec![' '; s.len()];
    idx.reverse();
    let mut i = 0;
    for ch in s.chars() {
        if vowels.contains(ch) {
            ret[idx[i]] = vow[i];
            i += 1;
        }
    }
    for j in 0..s.len() {
        if ret[j] == ' ' {
            ret[j] = s.as_bytes()[j] as char;
        }
    }
    ret.iter().collect::<String>()
}

// use stack to reverse
pub fn reverse_vowels_1(mut s: String) -> String {
    let vwls = "aeiouAEIOU";
    let mut stack: Vec<char> = s.chars().filter(|&c| vwls.contains(c)).collect();
    s.chars()
        .map(|c| if vwls.contains(c) { stack.pop().unwrap() } else { c })
        .collect()
}

// use set collect vowels
pub fn reverse_vowels_2(mut s: String) -> String {
    let vwls: HashSet<char> = HashSet::from(['a','e','i','o','u','A','E','I','O','U']);
    let mut stack: Vec<char> = s.chars().filter(|c| vwls.contains(c)).collect();
    s.chars()
        .map(|c| if vwls.contains(&c) { stack.pop().unwrap() } else { c })
        .collect()
}