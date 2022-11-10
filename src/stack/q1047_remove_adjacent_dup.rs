pub fn remove_duplicates(s: String) -> String {
    let mut stk = vec![];
    for ch in s.chars() {
        if !stk.is_empty() && stk.last().unwrap() == &ch{
            stk.pop();
        } else {
            stk.push(ch);
        }
    }
    stk.into_iter().collect::<String>()
}

pub fn remove_duplicates_u8(s: String) -> String {
    let mut ss: Vec<u8> = Vec::new();
    for c in s.bytes() {
        if !ss.is_empty() && *ss.last().unwrap() == c {
            ss.pop();
        } else {
            ss.push(c);
        }
    }
    String::from_utf8(ss).unwrap()
}