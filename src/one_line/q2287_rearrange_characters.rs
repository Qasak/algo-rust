pub fn rearrange_characters(s: String, target: String) -> i32 {
    target
        .bytes()
        .fold(vec![0; 26], |mut v, b| {
            v[(b - b'a') as usize] += 1;
            v
        })
        .iter()
        .zip(
            s.bytes()
                .fold(vec![0; 26], |mut v, b| {
                    v[(b - b'a') as usize] += 1;
                    v
                })
                .iter(),
        )
        .filter(|(&i, &j)| i > 0)
        .map(|(i, j)| j / i)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::one_line::q2287_rearrange_characters::rearrange_characters;

    #[test]
    fn f() {
        let s = "lrnvlcqukanpdnluowenfxquitzryponxsikhciohyostvmkapkfpglzikitwiraqgchxnpryhwpuwpozacjhmwhjvslprqlnxrk".to_owned();
        let target = "woijih".to_owned();
        assert_eq!(2, rearrange_characters(s, target));
    }
}
