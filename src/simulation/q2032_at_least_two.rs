use std::collections::HashSet;

pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let (set1, set2, set3) = (get_set(&nums1), get_set(&nums2), get_set(&nums3));
    set1.intersection(&set2).
        chain(set1.intersection(&set3).
            chain(set2.intersection(&set3))).map(|&x| x).
        collect::<HashSet<_>>().into_iter().
        collect::<Vec<i32>>()
}


fn get_set(nums: &Vec<i32>) -> HashSet<i32> {
    nums.into_iter().fold(HashSet::new(), |mut set, &i| {
        set.insert(i);
        set
    })
}