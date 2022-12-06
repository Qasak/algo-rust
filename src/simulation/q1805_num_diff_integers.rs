use std::collections::HashSet;
pub fn num_different_integers(word: String) -> i32 {
    let mut cs = word.chars().collect::<Vec<char>>();
    let (mut j, n) = (0, cs.len());
    let mut set = HashSet::new();
    while j < n {
        let mut num = "".to_string();
        let mut flag = false;
        while j < n && cs[j].is_ascii_digit() {
            if cs[j] != '0' {
                flag = true;
            }
            if flag {
                num += cs[j].to_string().as_str();
            }
            j += 1;
        }
        if !flag && j > 0 && cs[j - 1] == '0' {
            set.insert("0".to_string());
        }
        if num.len() != 0 {
            set.insert(num);
        }
        j += 1;
    }
    set.len() as i32
}

// use trim_start_matches api: it returns &str
pub fn num_different_integers_1(word: String) -> i32 {
    let mut set = HashSet::new();
    let mut num = "".to_string();
    for c in word.chars() {
        if c.is_ascii_digit() {
            num.push(c);
        } else if num.len() != 0 {
            num.push('x');
            set.insert(num.trim_start_matches('0').to_string());
            num = "".to_string();
        }
    }
    if num.len() != 0 {
        num.push('x');
        set.insert(num.trim_start_matches('0').to_string());
    }
    set.len() as i32
}