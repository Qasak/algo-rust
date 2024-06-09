use std::cmp::max;
fn dfs(l: usize, r: usize, nums: &Vec<i32>, cache: &mut Vec<Vec<i32>>) -> i32 {
    if l >= r - 1 {
        return 0;
    }
    if cache[l][r] != -1 {
        return cache[l][r];
    }
    for i in l + 1..r {
        let coins = nums[l] * nums[i] * nums[r] + dfs(l, i, nums, cache) + dfs(i, r, nums, cache);
        cache[l][r] = max(cache[l][r], coins);
    }
    return cache[l][r];
}

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums_new = vec![1; n + 2];
        for i in 0..n {
            nums_new[i + 1] = nums[i];
        }
        let mut cache = vec![vec![-1; n + 2]; n + 2];
        return dfs(0, n + 1, &nums_new, &mut cache);
    }
}