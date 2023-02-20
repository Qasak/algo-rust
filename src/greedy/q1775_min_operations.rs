use std::collections::BinaryHeap;
use std::mem::swap;

// double ptr solution
pub fn min_operations(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
    fn get_cnt(nums1: &Vec<i32>, nums2: &Vec<i32>, sum1: i32, sum2: i32) -> i32 {
        let mut ret = 0;
        let mut diff = sum1 - sum2;
        let (n, m) = (nums1.len(), nums2.len());
        let (bound1, bound2) = ((n, n * 6), (m, m * 6));
        if bound1.0 > bound2.1 || bound1.1 < bound2.0 {
            -1
        } else {
            let (mut i, mut j) = (n - 1, 0);
            while diff > 0 && i >= 0 && j < m {
                if nums1[i] - 1 > 6 - nums2[j] {
                    diff -= (nums1[i] - 1);
                    i -= 1;
                } else {
                    diff -= (6 - nums2[j]);
                    j += 1;
                }
                ret += 1;
            }
            while diff > 0 && i >= 0 {
                diff -= (nums1[i] - 1);
                i -= 1;
                ret += 1;
            }
            while diff > 0 && j < m {
                diff -= (6 - nums2[j]);
                j += 1;
                ret += 1;
            }
            ret
        }
    }
    nums1.sort();
    nums2.sort();
    let (sum1, sum2) = (nums1.iter().sum::<i32>(), nums2.iter().sum::<i32>());
    if sum1 > sum2 {
        get_cnt(&nums1, &nums2, sum1, sum2)
    } else if sum1 < sum2 {
        get_cnt(&nums2, &nums1, sum2, sum1)
    } else {
        0
    }
}

// BinaryHeap Solution
pub fn min_operations_1(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    let (mut n, mut m) = (a.len(), b.len());
    if n * 6 < m || m * 6 < n {
        return -1;
    }
    let (mut sum1, mut sum2): (i32, i32) = (a.iter().sum(), b.iter().sum());
    let mut pq = BinaryHeap::new();
    if sum1 < sum2 {
        swap(&mut sum1, &mut sum2);
        swap(&mut a, &mut b);
    }
    a.iter().for_each(|&i| pq.push(i - 1));
    b.iter().for_each(|&i| pq.push(6 - i));
    let (mut ret, mut diff) = (0, sum1 - sum2);
    while diff > 0 {
        ret += 1;
        diff -= pq.pop().unwrap();
    }
    ret
}
