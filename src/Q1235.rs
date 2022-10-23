pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let n = start_time.len();
    let mut jobs = (0..n).map(|i| vec![start_time[i], end_time[i], profit[i]]).collect::<Vec<_>>();
    jobs.sort_by(|a, b| a[1].cmp(&b[1]));
    let mut f = (0..n).map(|i| jobs[i][2]).collect::<Vec<_>>();
    let mut ret = 0;
    // 大于等于x的最小值[)
    // 小于等于x的最大值(] √
    // Minimum value greater than or equal to x [)
    // maximum value Less than or equal to x    (] √
    fn bs(jobs: &Vec<Vec<i32>>, index: usize) -> Option<usize> {
        let index = index as i32;
        let (mut l, mut r) = (-1, index - 1);
        while l < r {
            let m = l + ((r - l + 1) >> 1);
            if jobs[m as usize][1] > jobs[index as usize][0] {
                r = m - 1;
            } else {
                l = m;
            }
        }
        if l == -1 {None} else {Some(l as usize)}
    }
    for i in 1..n {
        // not select job i
        f[i] = f[i].max(f[i - 1]);
        // select job i，and find most close but not cross job j
        if let Some(j) = bs(&jobs, i) {
            f[i] = f[i].max(f[j] + jobs[i][2]);
        }
        ret = ret.max(f[i]);
    }
    ret
}