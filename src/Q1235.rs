pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let n = start_time.len();
    let mut jobs = (0..n).map(|i| vec![start_time[i], end_time[i], profit[i], profit[i]]).collect::<Vec<_>>();
    let mut ret = 0;
    fn bs(jobs: &Vec<Vec<i32>>, index: usize) -> usize {
        let (mut l, mut r) = (0, index);
        while l < r {
            let m = l + ((r - l) >> 1) as i32;
            if jobs[m as usize][1] <= jobs[index][0] {
                if jobs[m as usize + 1][1] <= jobs[index][0] {
                    l = m + 1;
                } else {
                    return m as usize;
                }
            } else {
                r = m - 1;
            }
        }
        l
    }
    for i in 1..n {
        
    }

    ret

}