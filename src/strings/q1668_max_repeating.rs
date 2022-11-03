// use contains
pub fn max_repeating(sequence: String, word: String) -> i32 {
    let (mut cnt, mut ret, mut cur) = (1, 0, String::from(word.as_str()));
    while cur.len() <= sequence.len() {
        let x = &cur;
        if sequence.contains(&cur) {
            ret = cnt;
        }
        cur.push_str(word.as_str());
        cnt += 1;
    }
    ret
}

// hand write O(nm) contains
pub fn max_repeating_1(sequence: String, word: String) -> i32 {
    fn contains(a: &String, b: &String) -> bool {
        if a.len() < b.len() {
            return contains(b, a);
        }
        let (mut it1, mut it2) = (a.chars().skip(0), b.chars());
        let mut skip = 0;
        loop {
            match (it1.next(), it2.next()) {
                (Some(ch1), Some(ch2)) => if ch1 != ch2 { skip += 1; it1 = a.chars().skip(skip);it2 = b.chars()} ,
                (_, None) => return true,
                (None, _) => return false
            }
        }
    }
    let (mut cnt, mut ret, mut cur) = (1, 0, String::from(word.as_str()));
    while cur.len() <= sequence.len() {
        if contains(&sequence, &cur) {
            ret = cnt;
        }
        cur.push_str(word.as_str());
        cnt += 1;
    }
    ret
}
