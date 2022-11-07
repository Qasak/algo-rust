pub fn ambiguous_coordinates(s: String) -> Vec<String> {
    /// add `.` before idx
    fn create_coord(arr: &str, idx: usize) -> Option<String> {
        if idx == 0 {
            // has prefix zero(s) => None
            if arr.len() > 1 && arr.chars().nth(0) == Some('0') {None} else {Some(String::from(arr))}
        } else {
            // pre has prefix zero(s) or suf has suffix zero(s) => None
            if arr[0..idx].len() > 1 && arr.chars().nth(0) == Some('0') || arr.chars().nth(arr.len() - 1) == Some('0') {None} else {Some(format!("{}.{}", &arr[0..idx], &arr[idx..arr.len()]))}
        }
    }
    let v = &s[1..s.len() - 1];
    let mut ret = vec![];
    // split v to two coordinates
    for i in 1..v.len() {
        let (x, y) = (&v[0..i], &v[i..v.len()]);
        // add `.` to each coordinate
        for p in 0..x.len() {
            for q in 0..y.len() {
                if let (Some(x_co), Some(y_co)) = (create_coord(x, p), create_coord(y, q)) {
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