pub fn longest_palindrome(s: String) -> String {
    let cs = s.chars().collect::<Vec<_>>();
    let mut ret: &[char] = &[];
    let mut max_len = 0;
    for i in 0..cs.len() {
        let (palin_1, palin_2) = (check(&cs, i, i), check(&cs, i, i + 1));
        let palin = if palin_1.len() > palin_2.len() {palin_1} else {palin_2};
        if palin.len() > max_len {
            ret = palin;
            max_len = palin.len();
        }
    }
    ret.iter().collect()
}
fn check(cs: &[char], mut i: usize, mut j: usize) -> &[char] {
    let (mut i, mut j) = (i as i32, j as i32);
    while i >= 0 && j < cs.len() as i32 && cs[i as usize] == cs[j as usize] {
        i -= 1;
        j += 1;
    }
    &cs[(i as usize + 1)..(j as usize)]
}