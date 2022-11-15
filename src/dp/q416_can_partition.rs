// 01 knapsack
pub fn can_partition(nums: Vec<i32>) -> bool {
    let mut sum = nums.iter().sum::<i32>() as usize;
    if sum % 2 == 1 {return false;}
    let target = sum / 2;
    let mut f = vec![false; target + 1];
    f[0] = true;
    for i in 0..nums.len() {
        for s in (1..(target + 1)).rev() {
            if s < nums[i] as usize {continue;}
            f[s] |= f[s - nums[i] as usize];
        }
    }
    f[target]
}