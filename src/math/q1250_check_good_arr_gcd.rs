use std::ops::Rem;
fn is_good_array(nums: Vec<i32>) -> bool {
    fn gcd<T: Rem<Output = T> + std::cmp::PartialOrd + From<u8> + Copy>(a: T, b: T) -> T {
        if b == T::from(0) {
            a
        } else {
            gcd(b, a % b)
        }
    }
    // 计算所有数的最大公因数
    let mut g = nums[0];
    for i in 1..nums.len() {
        g = gcd(g, nums[i]);
    }
    g == 1
}
