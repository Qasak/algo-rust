// bf: merge two arr
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut num = vec![nums1, nums2].concat();
    num.sort();
    let n = num.len();
    if n % 2 == 1 {
        num[(n - 1) / 2] as f64
    } else {
        (num[n / 2 - 1] + num[n / 2]) as f64 / 2.0
    }
}
// bf: virtual merge two arr
pub fn find_median_sorted_arrays_1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (n1, n2) = (nums1.len(), nums2.len());
    let n = n1 + n2;
    let (mut i, mut j) = (0, 0);
    let (mut mid1, mut mid2) = (0, 0);
    for _ in 0..=(n / 2) {
        mid1 = mid2;
        if j >= n2 || (i < n1 && nums1[i] < nums2[j]) {
            mid2 = nums1[i];
            i += 1;
        } else {
            mid2 = nums2[j];
            j += 1;
        }
    }
    if n % 2 == 1 {
        mid2 as f64
    } else {
        (mid1 + mid2) as f64 / 2.0
    }
}

// nums1[0..i-1]：数组 nums1 的左半部分；
// nums1[i..m-1]：数组 nums1 的右半部分；
// nums2[0..j-1]：数组 nums2 的左半部分；
// nums2[j..n-1]：数组 nums2 的右半部分。
// 分割点+二分： 约束：两个分割点，四个数之间的大小关系
// 即：nums1[i-1] <= nums2[j] && nums2[j-1] <= nums1[i]
// 当 m+n 是奇数时，中位数是两个有序数组中的第 (m+n)/2 个元素，
// 当 m+n 是偶数时，中位数是两个有序数组中的第 (m+n)/2 个元素和第 (m+n)/2+1 个元素的平均值
pub fn find_median_sorted_arrays_2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (n, m) = (nums1.len(), nums2.len());
    if n > m {
        return find_median_sorted_arrays(nums2, nums1);
    }
    let (mut l, mut r) = (0, n);
    let half_len = (n + m + 1) / 2;
    while l <= r {
        let i = (l + r) / 2;
        let j = half_len - i;
        if i > 0 && nums1[i - 1] > nums2[j] {
            r = i - 1;
        } else if i < n && nums2[j - 1] > nums1[i] {
            l = i + 1;
        } else {
            let max_left = if i == 0 {
                nums2[j - 1]
            } else if j == 0 {
                nums1[i - 1]
            } else {
                nums1[i - 1].max(nums2[j - 1])
            };
            if (m + n) % 2 == 1 {
                return max_left as f64;
            }
            let min_right = if i == n {
                nums2[j]
            } else if j == m {
                nums1[i]
            } else {
                nums1[i].min(nums2[j])
            };
            return (max_left + min_right) as f64 / 2.0;
        }
    }
    0.0
}
