// naive double ptr
pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let (mut l, mut r) = (0, n - 1);
    let (mut max_l, mut max_r) = (0, 0);
    let mut ret = 0;
    while l < r {
        max_l = max_l.max(height[l]);
        max_r = max_r.max(height[r]);
        if max_l < max_r {
            ret += max_l - height[l];
            l += 1;
        } else {
            ret += max_r - height[r];
            r -= 1;
        }
    }
    ret
}