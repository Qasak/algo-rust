use std::cmp::max;
fn dfs(l: usize, r: usize, cache: &mut Vec<Vec<i32>>, nums: &Vec<i32>, target: i32) -> i32 {
    let n = cache.len();
    if l >= r || l >= n || r >= n {
        return 0;
    }
    if cache[l][r] != -1 {
        // println!("hit cache!, l:{}, r:{}", l, r);
        return cache[l][r];
    }
    let mut res = vec![];
    if nums[l] + nums[r] == target {
        res.push(dfs(l + 1, r - 1, cache, nums, target));
    }
    if nums[l] + nums[l + 1] == target {
        res.push(dfs(l + 2, r, cache, nums, target));
    }
    if nums[r] + nums[r - 1] == target {
        res.push(dfs(l, r - 2, cache, nums, target));
    }
    // 因为迭代器生成的是对元素的引用，
    // 所以 max 返回的是一个包含最大值引用的 Option 类型，
    // 具体来说是 Option<&i32>
    // 如果 numbers 不为空，unwrap 将返回 &i32 类型的引用。
    // println!("res len: {}, target {}", res.len(), target);
    if res.len() == 0 {
        cache[l][r] = 0;
        return 0;
    }
    cache[l][r] = 1 + *res.iter().max().unwrap();
    cache[l][r]
}
impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 2 {
            return 1;
        }
        let mut cache = vec![vec![-1; n]; n];
        let a = 1 + dfs(1, n - 2, &mut cache, &nums, nums[0] + nums[n - 1]);
        let b = 1 + dfs(2, n - 1, &mut cache, &nums, nums[0] + nums[1]);
        let c = 1 + dfs(0, n - 3, &mut cache, &nums, nums[n - 1] + nums[n - 2]);
        return max(max(a, b), c);
    }

}