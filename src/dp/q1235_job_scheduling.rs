pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let n = start_time.len();
    let mut jobs = (0..n).map(|i| vec![start_time[i], end_time[i], profit[i]]).collect::<Vec<_>>();
    jobs.sort_by(|a, b| a[1].cmp(&b[1]));
    // define f[i] as the maximum profit for the first i jobs sorted by end time
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

// dfs solution
pub fn job_scheduling_1(st: Vec<i32>, et: Vec<i32>, pr: Vec<i32>) -> i32 {
    let mut jobs: Vec<(i32,i32,i32)> = st.into_iter()
        .zip(et.into_iter())
        .zip(pr.into_iter())
        .map(|((s,e),p)| (s,e,p))
        .collect();

    jobs.sort();

    let mut dp = vec![-1;jobs.len()];

    fn dfs(i: usize, jobs: &Vec<(i32,i32,i32)>, dp: &mut Vec<i32>) -> i32 {
        if i >= jobs.len() { return 0; }
        if dp[i] >= 0      { return dp[i]; }

        let k = jobs.partition_point(|&job| job.0 < jobs[i].1);
        dp[i] = dfs(i+1, jobs, dp).max(jobs[i].2 + dfs(k, jobs, dp));
        return dp[i];
    }

    return dfs(0, &jobs, &mut dp);
}
