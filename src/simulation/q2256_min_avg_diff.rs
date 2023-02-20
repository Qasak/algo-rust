pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut pre_sums = vec![0 as i64; n + 1];
    for i in 0..n {
        pre_sums[i + 1] = pre_sums[i] + nums[i] as i64;
    }
    let mut min_avg = i64::MAX;
    let mut min_index = n;

    for i in 0..nums.len() {
        let front_avg = pre_sums[i + 1] / (i as i64 + 1);

        let back_avg = if i == n - 1 {
            0
        } else {
            (pre_sums[n] - pre_sums[i + 1]) / (n as i64 - i as i64 - 1)
        };

        let avg_diff = (front_avg - back_avg).abs();

        if avg_diff < min_avg {
            min_avg = avg_diff;
            min_index = i;
        }
    }

    min_index as i32
}
