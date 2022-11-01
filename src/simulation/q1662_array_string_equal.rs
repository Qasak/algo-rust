pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    word1.join("") == word2.join("")
}


pub fn array_strings_are_equal_1(word1: Vec<String>, word2: Vec<String>) -> bool {
    let (mut i, mut j, mut p, mut q) = (0, 0, 0, 0);
    while i < word1.len() && j < word2.len() {
        while p < word1[i].len() && q < word2[j].len() {
            if word1[i].as_bytes()[p] != word2[j].as_bytes()[q] {
                return false;
            }
            p += 1;
            q += 1;
        }
        if p == word1[i].len() {
            i += 1;
            p = 0;
        }
        if q == word2[j].len() {
            j += 1;
            q = 0;
        }
    }
    i == word1.len() && j == word2.len()
}

pub fn array_strings_are_equal_2(word1: Vec<String>, word2: Vec<String>) -> bool {
    let mut it1 = word1.into_iter().flat_map(|s| s.chars().collect::<Vec<_>>());
    let mut it2 = word2.into_iter().flat_map(|s| s.chars().collect::<Vec<_>>());
    loop {
        match (it1.next(), it2.next()) {
            (Some(a), Some(b)) => {if a != b {return false;}}
            (None, None) => return true,
            _ => return false
        }
    }
}
