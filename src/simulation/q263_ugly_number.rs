pub fn is_ugly(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    for i in vec![2, 3, 5] {
        while n % i == 0 {
            n /= i;
        }
    }
    return n == 1;
}
