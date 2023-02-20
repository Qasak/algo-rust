pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let (mut i, mut j) = (1, 1);
    while i < nums.len() {
        if nums[i - 1] != nums[i] {
            nums[j] = nums[i];
            j += 1;
        }
        i += 1;
    }
    j as i32
}
