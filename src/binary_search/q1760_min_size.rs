pub fn minimum_size(nums: Vec<i32>, ops: i32) -> i32 {
    fn check(mut ops: i32, nums: &Vec<i32>, m: i32) -> bool {
        for n in nums {
            ops -= (n - 1) / m;
        }
        ops >= 0
    }
    let max = *nums.iter().max().unwrap();
    let (mut l, mut r) = (1, max);
    while l < r {
        let m = l + (r - l) / 2;
        if check(ops, &nums, m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}
