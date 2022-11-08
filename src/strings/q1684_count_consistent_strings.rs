use std::collections::HashSet;

// use HashSet
pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let set = allowed.chars().collect::<HashSet<_>>();
    words.into_iter().filter(|word| word.chars().all(|c| set.contains(&c))).count() as i32
}

// use bool array
pub fn count_consistent_strings_1(allowed: String, words: Vec<String>) -> i32 {
    let set = allowed.bytes().fold([false; 26], |mut set, ch| {set[(ch - b'a') as usize] = true; set});
    words.into_iter().filter(|word| word.bytes().all(|ch| set[(ch - b'a') as usize])).count() as i32
}