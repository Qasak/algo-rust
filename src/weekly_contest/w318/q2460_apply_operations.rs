pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut i = 0;
    while i < (n - 1) {
        if nums[i] == nums[i + 1] {
            nums[i] *= 2; nums[i + 1] = 0;
            i += 2;
        } else {i += 1;}
    }
    let mut ret = vec![0; n];
    i = 0;
    for num in nums {
        if num != 0 {ret[i] = num; i += 1};
    }
    ret
}