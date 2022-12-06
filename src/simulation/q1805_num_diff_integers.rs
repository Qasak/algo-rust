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
