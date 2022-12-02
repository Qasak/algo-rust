pub fn close_strings(word1: String, word2: String) -> bool {
    let (mut cnt1, mut cnt2, k1, k2) = (
        word1.bytes().fold(vec![0; 26], |mut cnt, b| {
            cnt[(b - b'a') as usize] += 1;
            cnt
        }),
        word2.bytes().fold(vec![0; 26], |mut cnt, b| {
            cnt[(b - b'a') as usize] += 1;
            cnt
        }),
        word1.bytes().fold(vec![false; 26], |mut k, b| {
            k[(b - b'a') as usize] = true;
            k
        }),
        word2.bytes().fold(vec![false; 26], |mut k, b| {
            k[(b - b'a') as usize] = true;
            k
        })
    );
    cnt1.sort(); cnt2.sort();
    cnt1 == cnt2 && k1 == k2
}