pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    // A[0] > A[1] < A[2] > A[3] < A[4] > ...
    let mut f = 0;
    // A[0] < A[1] > A[2] < A[3] > A[4] < ...
    let mut g = 0;
    for i in 0..n {
        let l = if i == 0 { i32::MAX } else { nums[i - 1] };
        let r = if i == n - 1 { i32::MAX } else { nums[i + 1] };
        match i % 2 {
            1 => f += 0.max(nums[i] - (l.min(r) - 1)),
            _ => g += 0.max(nums[i] - (l.min(r) - 1)),
        };
    }
    f.min(g)
}
