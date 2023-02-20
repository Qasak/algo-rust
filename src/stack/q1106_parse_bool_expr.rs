pub fn parse_bool_expr(expression: String) -> bool {
    let mut stk = vec![];
    for ch in expression.chars() {
        if ch == ',' {
            continue;
        }
        if ch != ')' {
            stk.push(ch);
            continue;
        }
        // ch == ')'
        let (mut f, mut t) = (0, 0);
        while let Some(val) = stk.pop() {
            if val == '(' {
                break;
            }
            if val == 'f' {
                f += 1
            } else {
                t += 1
            };
        }
        if let Some(exp) = stk.pop() {
            if exp == '!' {
                stk.push(if t == 1 { 'f' } else { 't' });
            } else if exp == '&' {
                stk.push(if f >= 1 { 'f' } else { 't' });
            } else {
                stk.push(if t >= 1 { 't' } else { 'f' });
            }
        }
    }
    if let Some(val) = stk.pop() {
        if val == 't' {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use crate::stack::q1106_parse_bool_expr::parse_bool_expr;

    #[test]
    fn t() {
        assert_eq!(false, parse_bool_expr("&(|(f))".to_string()));
        assert_eq!(true, parse_bool_expr("|(f,f,f,t)".to_string()));
        assert_eq!(true, parse_bool_expr("!(&(f,t))".to_string()));
    }
}
