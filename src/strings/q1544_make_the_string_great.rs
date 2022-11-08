use std::collections::VecDeque;
pub fn make_good(s: String) -> String {
    let mut stk = VecDeque::new();
    for ch in s.chars() {
        if let Some(&p) = stk.back() {
            if ch as u8 + 32 == p as u8 || ch as u8 - 32 == p as u8 {
                stk.pop_back();
            } else {
                stk.push_back(ch);
            }
        } else {
            stk.push_back(ch);
        }
    }
    stk.into_iter().collect::<String>()
}

pub fn make_good_1(s: String) -> String {
    let mut ss = Vec::new();
    for c in s.bytes(){
        if !ss.is_empty() && ss.last().unwrap() ^ c == 32 {
            ss.pop();
        } else { ss.push(c); }
    }
    String::from_utf8(ss).unwrap()
}