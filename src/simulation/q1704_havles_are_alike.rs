pub fn halves_are_alike(s: String) -> bool {
    let n = s.len();
    let a = &s.as_bytes()[..(n / 2)];
    let b = &s.as_bytes()[(n / 2)..n];
    fn cnt(s :&[u8]) -> i32 {
        let vowels = "aeiouAEIOU";
        let mut ret = 0;
        for c in s {
            if vowels.contains(*c as char) {
                ret += 1;
            }
        }
        ret
    }
    cnt(a) == cnt(b)
}


pub fn halves_are_alike_1(s: String) -> bool {
    let f = |s: &str| s.chars().filter(|&c| "aeiouAEIOU".contains(c)).count();
    f(&s[..s.len() / 2]) == f(&s[s.len() / 2..])
}