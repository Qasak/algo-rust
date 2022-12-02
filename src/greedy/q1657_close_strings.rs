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