// Vec and index
pub fn merge_alternately_0(word1: String, word2: String) -> String {
    let n1 = word1.len();
    let n2 = word2.len();
    let s1 = word1.bytes().collect::<Vec<_>>();
    let s2 = word2.bytes().collect::<Vec<_>>();
    let mut s = vec![];
    let mut i = 0;
    while i < n1.min(n2) {
        s.push(s1[i]);
        s.push(s2[i]);
        i += 1;
    }
    while i < n1 {
        s.push(s1[i]);
        i += 1;
    }
    while i < n2 {
        s.push((s2[i]));
        i += 1;
    }
    String::from_utf8(s).unwrap()
}

// iter and String
pub fn merge_alternately_1(word1: String, word2: String) -> String {
    let mut i1 = word1.chars().peekable();
    let mut i2 = word2.chars().peekable();
    let mut ret = "".to_string();
    while i1.peek().is_some() || i2.peek().is_some() {
        if let Some(ch) = i1.next() {
            ret.push(ch)
        };
        if let Some(ch) = i2.next() {
            ret.push(ch)
        };
    }
    ret
}
