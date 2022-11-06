// manually rotate
pub fn orderly_queue(s: String, k: i32) -> String {
    let k = k as usize;
    let mut bs = s.chars().collect::<Vec<char>>();
    if k > 1 {
        bs.sort();
        bs.into_iter().collect::<String>()
    } else {
        let mut cur = bs.iter().map(|ch| *ch).collect::<String>();
        for i in 1..s.len() {
            cur = cur.min(
                bs[i..s.len()].iter().map(|b| *b)
                    .chain(bs[0..i].iter().map(|b| *b))
                    .collect::<String>()
            );
        }
        cur
    }
}


// use .windows()
pub fn orderly_queue_1(s: String, k: i32) -> String {
    let k = k as usize;
    let mut bs = s.chars().collect::<Vec<char>>();
    if k > 1 {
        bs.sort();
        bs.into_iter().collect::<String>()
    } else {
        let tmp = s.chars().chain(s.chars()).collect::<Vec<_>>();
        let mut v = tmp.windows(s.len()).collect::<Vec<_>>();
        v.sort();

        // same as: v[0].iter().copied().collect::<_>()
        v[0].iter().map(|ch| *ch).collect::<_>()
    }
}

#[cfg(test)]
mod test {
    use crate::strings::q899_orderly_queue::orderly_queue;

    #[test]
    fn f() {
        assert_eq!("aacab".to_string(), orderly_queue("baaca".to_string(), 1));
        assert_eq!("aaabc".to_string(), orderly_queue("baaca".to_string(), 3));
    }

}