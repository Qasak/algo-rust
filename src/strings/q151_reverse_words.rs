// manual handle
pub fn reverse_words(s: String) -> String {
    let mut cs = s.chars().collect::<Vec<char>>();
    let mut ret = vec![];
    let mut i = 0;
    while i < cs.len() {
        while i < cs.len() && cs[i] == ' ' {
            i += 1;
            continue;
        }
        let mut cur = vec![];
        while i < cs.len() && cs[i] != ' ' {
            cur.push(cs[i]);
            i += 1;
        }
        ret.push(cur.into_iter().collect::<String>());
    }
    let mut ans = vec![];
    ret.reverse();
    for i in 0..ret.len() {
        for c in ret[i].chars() {
            ans.push(c);
        }
        if ret[i].len() > 0 && i != ret.len() - 1 {
            ans.push(' ');
        }
    }
    ans.into_iter().collect()
}

// api
pub fn reverse_words_api(s: String) -> String {
    // same as `s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")`
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}
