pub fn interpret(command: String) -> String {
    let cs = command.chars().collect::<Vec<char>>();
    let mut ret = vec![];
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == 'G' {
            ret.push('G');
            i += 1;
        } else {
            if cs[i + 1] == ')' {
                ret.push('o');
                i += 2;
            } else {
                ret.push('a'); ret.push('l');
                i += 4;
            }
        }
    }
    ret.into_iter().collect::<String>()
}

// use str.replace
pub fn interpret_1(command: String) -> String {
    command.replace("()", "o").replace("(al)", "al")
}
