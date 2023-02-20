use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stk = vec![];
    for c in s.chars() {
        match c {
            ')' => {
                if stk.is_empty() || *stk.last().unwrap() != '(' {
                    return false;
                } else {
                    stk.pop();
                }
            }
            ']' => {
                if stk.is_empty() || *stk.last().unwrap() != '[' {
                    return false;
                } else {
                    stk.pop();
                }
            }
            '}' => {
                if stk.is_empty() || *stk.last().unwrap() != '{' {
                    return false;
                } else {
                    stk.pop();
                }
            }
            _ => {
                stk.push(c);
            }
        }
    }
    stk.is_empty()
}

pub fn is_valid_1(s: String) -> bool {
    let mut stack = vec!['0'];
    for c in s.chars() {
        match c {
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return false;
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return false;
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return false;
                }
            }
            _ => stack.push(c),
        }
    }
    stack.len() == 1
}

// use HashMap
pub fn is_valid_2(s: String) -> bool {
    let mut stk = vec![];
    let pairs = [('(', ')'), ('[', ']'), ('{', '}')]
        .iter()
        .cloned()
        .collect::<HashMap<char, char>>();

    for c in s.chars() {
        if pairs.contains_key(&c) {
            stk.push(c);
        } else if stk.is_empty() || pairs[&stk.last().unwrap()] != c {
            return false;
        } else {
            stk.pop();
        }
    }
    stk.is_empty()
}
