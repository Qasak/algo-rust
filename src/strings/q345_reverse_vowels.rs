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
    for ch in s.bytes() {
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