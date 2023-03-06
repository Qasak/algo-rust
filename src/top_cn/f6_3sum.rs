pub fn three_sum_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;
    use std::collections::HashSet;
    let mut i = 0;
    let n = nums.len();
    let mut ret = HashSet::new();
    nums.sort_unstable();
    for i in 0..n {
        let (mut l, mut r) = (i + 1, n - 1);
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            match sum.cmp(&0) {
                Ordering::Equal => {
                    ret.insert(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                }
                Ordering::Less => {
                    l += 1;
                }
                Ordering::Greater => {
                    r -= 1;
                }
            }
        }
    }
    ret.into_iter().collect()
}

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;
    let mut i = 0;
    let n = nums.len();
    let mut ret = vec![];
    nums.sort_unstable();
    for i in 0..n - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let (mut l, mut r) = (i + 1, n - 1);
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            match sum.cmp(&0) {
                Ordering::Equal => {
                    ret.push(vec![nums[i], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                }
                Ordering::Less => {
                    l += 1;
                }
                Ordering::Greater => {
                    r -= 1;
                }
            }
        }
    }
    ret
}
