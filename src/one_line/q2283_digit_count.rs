pub fn digit_count(num: String) -> bool {
    num.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .fold([0; 10], |mut v, i| {
            v[i] += 1;
            v
        })
        .iter()
        .zip(num.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .all(|(&c, i)| c == i)
}

#[cfg(test)]
mod test {
    use crate::one_line::q2283_digit_count::digit_count;

    #[test]
    fn test_zip() {
        let v = vec![0; 5];
        let w = vec![0; 2];
        // If two arrays are of different lengths, the shorter one prevails
        let e = v.iter().zip(w.iter()).all(|(i, j)| {
            println!("{:?}", (i, j));
            i == j
        });
        assert_eq!(true, e);
    }

    #[test]
    fn test_cnt() {
        assert_eq!(true, digit_count("1210".to_owned()));
        assert_eq!(false, digit_count("030".to_owned()));
    }
}
