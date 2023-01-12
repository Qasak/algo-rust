use std::collections::HashMap;

// use index
pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    let (cs, n, mut i, mut ret) = (s.chars().collect::<Vec<char>>(), s.len(), 0, String::new());
    let map = knowledge.iter().map(|v| (v[0].as_str(), v[1].as_str())).collect::<HashMap<_, _>>();
    while i < n {
        if cs[i] == '(' {
            i += 1;
            let mut t = String::new();
            while cs[i] != ')' {
                t.push(cs[i]);
                i += 1;
            }
            if let Some(replace) = map.get(t.as_str()) {
                ret.push_str(replace);
            } else {
                ret.push('?');
            }
        } else {
            ret.push(cs[i]);
        }
        i += 1;
    }
    ret
}