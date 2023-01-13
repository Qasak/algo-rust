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