use std::collections::HashMap;

// use index
pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    let (cs, n, mut i, mut map, mut ret) = (s.chars().collect::<Vec<char>>(), s.len(), 0, HashMap::new(), String::new());
    for p in knowledge.iter() {
        map.insert(p[0].as_str(), p[1].as_str());
    }
    while i < n {
        if cs[i] == '(' {
            i += 1;
            let mut t = String::new();
            while cs[i] != ')' {
                t.push(cs[i]);
                i += 1;
            }
            if let Some(replace) = map.get(&t.as_str()) {
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