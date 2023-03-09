pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let (mut l, mut r) = (0, n);
    while l < r {
        let m = l + (r - l) / 2;
        let cur = nums[m];
        if cur == target {
            return m as i32;
        }
        // if-else can merge
        if target >= nums[l] {
            if cur >= nums[l] {
                if cur > target {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else {
                r = m;
            }
        } else {
            if cur < nums[l] {
                if cur > target {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else {
                l = m + 1;
            }
        }
    }
    -1
}
