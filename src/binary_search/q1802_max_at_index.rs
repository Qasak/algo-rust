
fn check(m: i32, index: i32, n: i32) -> i64 {
    let m = m as i64;
    let (l, r) = (index as i64, (n - 1 - index) as i64);
    let a1r = m - r;
    let a1l = m - l;
    m + if m > l {get_sn(a1l as i64, l)} else {get_sn(1, m - 1) + l - m + 1} + if m > r {get_sn(a1r as i64, r)} else {get_sn(1, m - 1) + r - m + 1}
}
fn get_sn(a1: i64, n: i64) -> i64 {
    a1 * n + (n - 1) * n / 2
}
pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
    // 111111 = 6 + 1
    // 121111 = 7 + 3
    // 232111 = 10 + 4
    // 343211 = 14 + 5
    // 454321 = 19 + 6
    // 565432 = 25 + 6
    // 676543 = 31 + 6

    let (mut l, mut r) = (1, max_sum);
    while l < r {
        let m = l + (r - l + 1) / 2;
        let x = check(m, index, n);
        if x > max_sum as i64 {
            r = m - 1;
        } else {
            l = m;
        }
    }
    l
}
#[cfg(test)]
mod test {
    use crate::binary_search::q1802_max_at_index::{get_sn, max_value};

    #[test]
    fn f() {
        assert_eq!(6, get_sn(6, 1));
        assert_eq!(18, get_sn(3, 4));

        let m = 4_i64;
        let r = 4_i64;
        assert_eq!(7, get_sn(1, m - 1) + r - m + 1);

        let m = 3_i64;
        let r = 4_i64;
        assert_eq!(5, get_sn(1, m - 1) + r - m + 1);

        let (n, idx, max_sum) = (6, 0, 6);
        assert_eq!(1, max_value(n, idx, max_sum));

        let (n, idx, max_sum) = (6, 5, 1000000000);
        assert_eq!(166666669, max_value(n, idx, max_sum));

        let (n, idx, max_sum) = (666666, 321321, 99887766);
        assert_eq!(9961, max_value(n, idx, max_sum));

    }
}