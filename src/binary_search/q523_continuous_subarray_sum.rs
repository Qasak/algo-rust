use ::std::collections::HashMap;
pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut map = HashMap::new();
    map.insert(0, -1);
    let mut sum = 0;
    for (i, num) in nums.into_iter().enumerate() {
        sum += num;
        sum %= k;
        let pre_idx = *map.entry(sum).or_insert(i as i32);
        if i as i32 - pre_idx >= 2 {
            return true;
        }
    }
    false
}
