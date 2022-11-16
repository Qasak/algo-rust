

pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
    let n = nums.len();
    for i in 0..n {
        if (i as i32 - nums[i] as i32).abs() > 1 {return false;}
    }
    true
}