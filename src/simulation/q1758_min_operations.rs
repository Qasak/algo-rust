pub fn min_operations(s: String) -> i32 {
    let cnt = s.bytes().enumerate().filter(
        // Counting the number of even index == '0'
        |(i, ch)| (*ch - b'0' == (*i % 2) as u8)).count() as i32;
    cnt.min(s.len() as i32 - cnt)
}