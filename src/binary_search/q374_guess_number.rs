unsafe fn guess(num: i32) -> i32 {-1}

unsafe fn guess_number(n: i32) -> i32 {
    let (mut l, mut r) = (0 as u32, n as u32);
    while l < r {
        let m = l + (r - l + 1) / 2;
        let res = guess(m as i32);
        // too big
        if res == -1 {
            r = m - 1;
        } else {
            l = m;
        }
    }
    l as i32
}