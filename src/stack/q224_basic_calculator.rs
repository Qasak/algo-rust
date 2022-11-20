pub fn calculate(s: String) -> i32 {
    // -(a - (b - c))
    let (mut sign, mut stk, cs, n, mut i, mut ans) = (1, vec![], s.chars().collect::<Vec<char>>(), s.len(), 0, 0);
    stk.push(sign);
    while i < n {
        if cs[i] == ' ' {i += 1; continue;}
        else if cs[i] == '+' {sign = stk[stk.len() - 1];}
        else if cs[i] == '-' {sign = -stk[stk.len() - 1];}
        else if cs[i] == '(' {stk.push(sign);}
        else if cs[i] == ')' {stk.pop();}
        else {
            let mut cur: i32 = 0;
            while i < n && cs[i].is_ascii_digit() {
                cur *= 10; cur += (cs[i] as u8 - b'0') as i32; i += 1;
            }
            i -= 1;
            ans += sign * cur;
        }
        i += 1;
    }
    ans
}
