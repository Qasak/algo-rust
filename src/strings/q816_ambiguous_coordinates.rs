pub fn ambiguous_coordinates(s: String) -> Vec<String> {
    /// add `.` before idx
    fn create_frac(arr: &[char], idx: usize) -> Option<String> {
        if idx == 0 {
            let all = arr.iter().collect::<String>();
            let all_val: u32 = all.parse().unwrap();
            // println!("{all}, {all_val}");
            // has prefix zero(s)
            if all_val.to_string().len() != all.len() {
                None
            } else {
                Some(all)
            }
        } else {

            let (pre, suf) =(arr[0..idx].iter().collect::<String>(), arr[idx..arr.len()].iter().collect::<String>()) ;
            let pre_val :u32 = pre.parse().unwrap();
            let suf_val :u32 = suf.parse().unwrap();
            // println!("{pre}, {pre_val}, {suf}, {suf_val}");
            // pre has prefix zero(s) or suf has suffix zero(s) or pre == suf == 0
            if pre_val.to_string().len() != pre.len() || suf_val % 10 == 0 || (pre_val == 0) && (suf_val == 0) {
                None
            } else {
                Some(format!("{}.{}", pre, suf))
            }
        }
    }
    let mut s = s;
    let s = s.strip_prefix("(").unwrap();
    let s = s.strip_suffix(")").unwrap();
    let v = s.chars().collect::<Vec<_>>();
    let mut ret = vec![];
    // split v to two coordinate
    for i in 1..v.len() {
        let (x, y) = (&v[0..i], &v[i..v.len()]);
        // println!("{:?} {:?}", x, y);
        // add `.` to each coordinate
        for p in 0..x.len() {
            for q in 0..y.len() {
                if let (Some(x_co), Some(y_co)) = (create_frac(x, p), create_frac(y, q)) {
                    ret.push(format!("({}, {})", x_co, y_co));
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod test{
    use crate::strings::q816_ambiguous_coordinates::ambiguous_coordinates;

    #[test]
    fn f() {
        println!("{:?}", ambiguous_coordinates("(123)".to_string()));
        println!("{:?}", ambiguous_coordinates("(00011)".to_string()));
        println!("{:?}", ambiguous_coordinates("(0123)".to_string()));
        println!("{:?}", ambiguous_coordinates("(100)".to_string()));
        println!("{:?}", ambiguous_coordinates("(0010)".to_string()));
        println!("{:?}", ambiguous_coordinates("(0101)".to_string()));
    }

}