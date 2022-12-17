pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
    let (n, m) = (groups.len(), nums.len());
    let (mut i, mut ii) = (0, 0);
    let (mut l, mut r) = (0, 0);
    while i < n && r < m {
        let nn = groups[i].len();
        if groups[i][ii] == nums[r] {
            ii += 1;
            r += 1;
            if r - l == nn {
                ii = 0;
                i += 1;
                l = r;
            }
        } else {
            ii = 0;
            // r += 1;
            // l = r;
            l += 1;
            r = l;
        }
    }
    i == n
}