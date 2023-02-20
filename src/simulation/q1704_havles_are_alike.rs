use std::collections::HashSet;

pub fn halves_are_alike(s: String) -> bool {
    let n = s.len();
    fn cnt(s: &str) -> i32 {
        let vowels = "aeiouAEIOU";
        let mut ret = 0;
        for c in s.chars() {
            if vowels.contains(c) {
                ret += 1;
            }
        }
        ret
    }
    cnt(&s[..(n / 2)]) == cnt(&s[(n / 2)..n])
}

pub fn halves_are_alike_1(s: String) -> bool {
    let f = |s: &str| s.chars().filter(|&c| "aeiouAEIOU".contains(c)).count();
    f(&s[..s.len() / 2]) == f(&s[s.len() / 2..])
}

pub fn halves_are_alike_2(s: String) -> bool {
    let f = |s: &str| {
        let set: HashSet<char> = "aeiouAEIOU".chars().fold(HashSet::new(), |mut s, c| {
            s.insert(c);
            s
        });
        s.chars().filter(|c| set.contains(c)).count()
    };
    f(&s[..s.len() / 2]) == f(&s[s.len() / 2..])
}
