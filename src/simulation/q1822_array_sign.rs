pub fn array_sign(nums: Vec<i32>) -> i32 {
    if nums.contains(&0) { 0 } else {
        nums.iter().fold(1, |cnt, &num| if num > 0 { cnt * 1 } else { cnt * -1 })
    }
}
