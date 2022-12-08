pub fn is_valid(s: String) -> bool {
    let mut stack = vec!['0'];
    for c in s.chars() {
        match c {
            ')' => {if stack.pop().unwrap() != '(' {return false}},
            ']' => {if stack.pop().unwrap() != '[' {return false}},
            '}' => {if stack.pop().unwrap() != '{' {return false}},
            _ => {stack.push(c)},
        }
    }
    stack.len() == 1
}