pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort();
    // intervals.sort_unstable();
    // intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let n = intervals.len();
    let mut ret = vec![];
    let mut i = 0;
    while i < n {
        let mut cur_l = intervals[i][0];
        let mut cur_r = intervals[i][1];

        while i < n && cur_r >= intervals[i][0] {
            cur_r = cur_r.max(intervals[i][1]);
            i += 1;
        }
        ret.push(vec![cur_l, cur_r]);
    }
    ret
}


#[test]
fn test_merge() {
    let mut intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let mut ans = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
    assert_eq!(merge(intervals), ans);


    let mut intervals = vec![vec![1, 4], vec![4, 5]];
    let mut ans = vec![vec![1, 5]];
    assert_eq!(merge(intervals), ans);

    let mut intervals = vec![vec![1, 4], vec![0, 4]];
    let mut ans = vec![vec![0, 4]];
    assert_eq!(merge(intervals), ans);

    let mut intervals = vec![vec![1, 4], vec![0, 0]];
    let mut ans = vec![vec![0, 0], vec![1, 4]];
    assert_eq!(merge(intervals), ans);

    let mut intervals = vec![vec![1, 4], vec![2, 3]];
    let mut ans = vec![vec![1, 4]];
    assert_eq!(merge(intervals), ans);

    let mut intervals = vec![vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![1, 10]];
    let mut ans = vec![vec![1, 10]];
    assert_eq!(merge(intervals), ans);
}
