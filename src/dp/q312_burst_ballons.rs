// cached backtrack
pub fn max_coins(nums: Vec<i32>) -> i32 {
    fn dfs(l: usize, r: usize, nums: &Vec<i32>, f: &mut Vec<Vec<i32>>) -> i32 {
        if l + 1 >= r {
            return 0;
        }
        if f[l][r] != -1 {
            return f[l][r];
        }
        for i in (l + 1)..=(r - 1) {
            f[l][r] = f[l][r].max(nums[i] * nums[l] * nums[r] + dfs(l, i, nums, f) + dfs(i, r, nums, f));
        }
        return f[l][r];
    }
    let n = nums.len();
    let mut v = vec![0; n + 2];
    for i in 1..=n {
        v[i] = nums[i - 1];
    }
    v[0] = 1;
    v[n + 1] = 1;
    let mut f = vec![vec![-1; n + 2]; n + 2];
    dfs(0, n + 1, &v, &mut f)
}
