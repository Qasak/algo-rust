// naive for loop
pub fn rearrange_characters(s: String, target: String) -> i32 {
    let mut s_it = s.bytes();
    let mut ret = i32::MAX;
    let mut cnt = vec![0; 26];
    let mut contains = vec![false; 26];
    for c in target.bytes() {
        cnt[(c - b'a') as usize] += 1;
        contains[(c - b'a') as usize] = true;
    }
    let mut scnt = vec![0; 26];
    while let Some(c) = s_it.next() {
        scnt[(c - b'a') as usize] += 1;
    }
    for i in 0..26 {
        if contains[i] {
            ret = ret.min(scnt[i] / cnt[i]);
        }
    }
    ret
}

// iter version
pub fn rearrange_characters_1(s: String, target: String) -> i32 {
    let cnt1 = target.bytes().fold(vec![0; 26], |mut v, b| {
        v[(b - b'a') as usize] += 1;
        v
    });
    let cnt2 = s.bytes().fold(vec![0; 26], |mut v, b| {
        v[(b - b'a') as usize] += 1;
        v
    });
    cnt1.iter()
        .zip(cnt2.iter())
        .filter(|(&i, &j)| i > 0)
        .map(|(i, j)| j / i)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::strings::q2287_rerange_characters::{rearrange_characters, rearrange_characters_1};

    #[test]
    fn f() {
        let s = "lrnvlcqukanpdnluowenfxquitzryponxsikhciohyostvmkapkfpglzikitwiraqgchxnpryhwpuwpozacjhmwhjvslprqlnxrk".to_owned();
        let target = "woijih".to_owned();
        assert_eq!(2, rearrange_characters(s, target));
    }

    #[test]
    fn ff() {
        let s = "lrnvlcqukanpdnluowenfxquitzryponxsikhciohyostvmkapkfpglzikitwiraqgchxnpryhwpuwpozacjhmwhjvslprqlnxrk".to_owned();
        let target = "woijih".to_owned();
        assert_eq!(2, rearrange_characters_1(s, target));
    }
}
