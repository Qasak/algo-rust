use std::collections::HashMap;
pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut map = HashMap::new();
    let k = k as usize;
    let mut pre_sum = vec![0 as i64; nums.len() + 1];
    let mut ret = 0;
    for i in 0..nums.len() {
        pre_sum[i + 1] = pre_sum[i] + nums[i] as i64;
    }
    for i in 0..nums.len() {
        *map.entry(&nums[i]).or_insert(0) += 1;
        if i >= k {
            *map.entry(&nums[i - k]).or_insert(0) -= 1;
            if *map.get(&nums[i - k]).unwrap() == 0 {
                map.remove(&nums[i - k]);
            }
        }
        if i >= k - 1 && map.len() == k {
            ret = ret.max(pre_sum[i + 1] - pre_sum[i - k + 1]);
        }
    }
    ret
}
