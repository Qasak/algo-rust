pub fn shortest_subarray(mut nums: Vec<i32>, k: i32) -> i32 {
    use ::std::collections::VecDeque;
    // prefix sum + travers length from 1 to nums.len()
    // O(n^2)
    // minimum val of greater or equal problem
    // use dequeue
    let n = nums.len();
    // notice that, the sum of nums may overflow, so use i64 to calculate prefix sum
    let mut pre: Vec<i64> = vec![0; n + 1];
    let mut q = VecDeque::new();
    let mut ans = i32::MAX;
    for i in 0..n {
        pre[i + 1] = pre[i] + nums[i] as i64;
    }
    for i in 0..=n {
        let cur = pre[i];
        while !q.is_empty() && cur - pre[*q.front().unwrap() as usize] >= k as i64 {
            ans = ans.min(i as i32 - q.pop_front().unwrap());
        }
        while !q.is_empty() && pre[*q.back().unwrap() as usize] >= cur {
            q.pop_back();
        }
        q.push_back(i as i32);
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}
