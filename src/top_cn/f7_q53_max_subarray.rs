pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut ret = nums[0];
    let mut cur = ret;
    for i in 1..nums.len() {
        if cur < 0 {
            cur = nums[i];
        } else {
            cur += nums[i];
        }
        ret = ret.max(cur);
    }
    ret
}
