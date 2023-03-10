pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn lb(nums: &Vec<i32>, target: i32) -> usize {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
    let (n, ll, rr) = (nums.len(), lb(&nums, target), lb(&nums, target + 1));
    vec![
        if ll != n && nums[ll] == target {
            ll as i32
        } else {
            -1
        },
        if rr != 0 && nums[rr - 1] == target {
            (rr - 1) as i32
        } else {
            -1
        },
    ]
}

pub fn search_range_std_partition_point(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if let Err(l) = nums.binary_search(&target) {
        return vec![-1, -1];
    }
    let low = nums.partition_point(|x| x < &target);
    let high = nums.partition_point(|x| x <= &target) - 1;
    vec![low as i32, high as i32]
}