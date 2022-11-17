// imagine two line segments a and b on the x-axis,
// where a moves from the far side to b and then leaves
pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
    let mut ans = 0;
    let s_a = (ax2 - ax1) * (ay2 - ay1);
    let s_b = (bx2 - bx1) * (by2 - by1);
    ans +=  s_a + s_b;
    let l1 = (ax2 - bx1).min(bx2 - ax1).min(ax2 - ax1).min(bx2 - bx1);
    let l2 = (ay2 - by1).min(by2 - ay1).min(ay2 - ay1).min(by2 - by1);
    if s_a != 0 && s_b != 0 && l1 > 0 && l2 > 0 {
        ans -= l1 * l2;
    }
    ans
}