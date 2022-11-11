pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ret = 0;
    let n = height.len();
    let (mut l, mut r) = (0, n - 1);
    while l < r {
        ret = ret.max( (r - l) as i32 * (height[r].min(height[l])));
        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }
    ret
}