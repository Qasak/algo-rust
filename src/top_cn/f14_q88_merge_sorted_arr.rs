pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut i, mut j) = (m as usize, n as usize);
    let mut p = i + j - 1;
    while j > 0 {
        if i > 0 && nums1[i - 1] > nums2[j - 1] {
            i -= 1;
            nums1[p] = nums1[i];
        } else {
            j -= 1;
            nums1[p] = nums2[j];
        }
        p -= 1;
    }
}