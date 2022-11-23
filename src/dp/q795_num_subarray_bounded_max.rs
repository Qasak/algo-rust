pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    let mut i = 0;
    let n = nums.len();
    // f[i - 1]  the number of subarray ending with the index of i - i, that satisfy the max value in [left, right]
    let mut f = vec![0; n + 1];
    for j in 1..=n {
        if nums[j - 1] >= left && nums[j - 1] <= right {
            f[j] = j - i;
        }
        if nums[j - 1] > right {
            i = j;
        }
        if nums[j - 1] < left {
            f[j] = f[j - 1];
        }
    }
    f.into_iter().sum::<usize>() as i32
}
