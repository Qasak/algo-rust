use std::collections::HashSet;

// bf
pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
    let mut ret = -1;
    let mut set = HashSet::new();
    let mut max = 0;
    for &n in nums.iter() {
        max = max.max(n);
        set.insert(n);
    }
    nums.sort();
    for i in 0..nums.len() {
        let n = nums[i];
        let mut cur = n;
        let mut len = 0;
        while cur <= max && set.contains(&cur) {
            len += 1;
            set.remove(&cur);
            cur *= cur;
        }
        if len >= 2 {
            ret = ret.max(len as i32);
        }
    }
    ret
}
