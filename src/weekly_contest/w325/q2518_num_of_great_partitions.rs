pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
    let (mo, mut ret, n, sum, mut f) = (
        1e9 as i32 + 7,
        1,
        nums.len(),
        nums.iter().map(|&x| x as i64).sum::<i64>(),
        vec![0; k as usize],
    );
    if sum < 2 * k as i64 {
        return 0;
    }
    f[0] = 1;
    for x in nums {
        ret = ret * 2 % mo;
        for j in (x as usize..(k as usize)).rev() {
            f[j] = (f[j] + f[j - x as usize]) % mo;
        }
    }
    for u in f {
        ret = (ret - 2 * u % mo + mo) % mo;
    }
    ret
}

#[cfg(test)]
mod test {
    use crate::weekly_contest::w325::q2518_num_of_great_partitions::count_partitions;

    #[test]
    fn f() {
        let nums = vec![1, 2, 3, 4];
        let k = 4;
        let ret = count_partitions(nums, k);
        assert_eq!(ret, 6);
    }
}
