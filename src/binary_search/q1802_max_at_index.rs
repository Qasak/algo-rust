
fn check(m: i64, index: i64, n: i64) -> i64 {
    let (l, r) = (index, n - 1 - index);
    m + if m > l {get_sn(m - l, l)} else {get_sn(1, m - 1) + l - m + 1} + if m > r {get_sn(m - r, r)} else {get_sn(1, m - 1) + r - m + 1}
}
fn get_sn(a1: i64, n: i64) -> i64 {
    a1 * n + (n - 1) * n / 2
}

pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
    let (mut l, mut r) = (1, max_sum);
    while l < r {
        let m = l + (r - l + 1) / 2;
        if check(m as i64, index as i64, n as i64) > max_sum as i64 {
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