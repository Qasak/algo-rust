pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let (n, sum) = (nums.len() as i32, nums.iter().sum::<i32>());
    let target = sum - x;
    if target < 0 {
        return -1;
    } else if target == 0 {
        return n;
    }
    let (mut i, mut cur, mut max_len) = (0, 0, 0);
    for (j, &num) in nums.iter().enumerate() {
        cur += num;
        while cur > target {
            cur -= nums[i];
            i += 1;
        }
        if cur == target {
            max_len = max_len.max(j as i32 - i as i32 + 1);
        }
    }
    if max_len == 0 {-1} else {n - max_len as i32}
}