// https://github.com/Qasak/algo-rust
pub fn close_strings(word1: String, word2: String) -> bool {
    let (mut cnt1, mut cnt2) = (
        word1.bytes().fold((vec![0; 26], vec![false; 26]), |mut cnt, b| {
            cnt.0[(b - b'a') as usize] += 1;
            cnt.1[(b - b'a') as usize] = true;
            cnt
        }),
        word2.bytes().fold((vec![0; 26], vec![false; 26]), |mut cnt, b| {
            cnt.0[(b - b'a') as usize] += 1;
            cnt.1[(b - b'a') as usize] = true;
            cnt
        })
    );
    cnt1.0.sort(); cnt2.0.sort();
    cnt1 == cnt2
}

// https://leetcode.com/problems/determine-if-two-strings-are-close/solutions/1029085/rust-solution/
pub fn close_strings_1(word1: String, word2: String) -> bool {
    let mut counts1 = [0; 26];
    let mut counts2 = [0; 26];
    for &b in word1.as_bytes() {
        counts1[(b - b'a') as usize] += 1;
    }
    for &b in word2.as_bytes() {
        counts2[(b - b'a') as usize] += 1;
    }
    if (0..26).any(|i| (counts1[i] > 0) != (counts2[i] > 0)) {
        return false;
    }
    counts1.sort_unstable();
    counts2.sort_unstable();
    counts1 == counts2
}
