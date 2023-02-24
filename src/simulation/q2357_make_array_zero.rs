pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
    let mut cnt = 0 ;
    while let Some(min) = get_min(&nums) {
        nums = nums.iter()
            .map(|&n| if n > 0 {n - min} else {n})
            .collect();
        cnt += 1;
    }
    cnt
}

fn get_min(nums: &Vec<i32>) -> Option<i32> {
    nums.iter().filter(|&&n| n != 0).min().copied()
}
