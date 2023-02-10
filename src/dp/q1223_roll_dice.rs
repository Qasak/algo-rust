pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    // 三元祖定义状态
    // (i, last, left)
    // i: 剩余投掷次数
    // last: 上次数字
    // left: 上次数字可用次数
    // 定义dfs为(i, last, left)为起始状态的不同点数序列数量

    // 情况：
    // 1.当前数字与last相同: 消耗left
    // dfs(i - 1, last, left - 1)
    // 2.当前数字与last不同: 重制left
    // dfs(i - 1, j, roll_max[j] - 1)

    // 递归终点：
    // i == 0 return 1

    // 三元组cache
    // 「先掷 111 后掷 333」和「先掷 222 后掷 333」，都会递归到 dfs(n−2,3,rollMax[3]−1)\textit{dfs}(n-2,3,\textit{rollMax}[3]-1)dfs(n−2,3,rollMax[3]−1)
    // let mut cache = vec![vec![vec![-1; roll_max.max]; m]; n];
    fn dfs(i: i32, last: usize, left: i32, roll_max: &Vec<i32>, cache: &mut Vec<Vec<Vec<i64>>>) -> i64{
        const mo: i64 = 1e9 as i64 + 7;
        if i == 0 {
            return 1;
        }
        if cache[i as usize][last][left as usize] != -1 {
            return cache[i as usize][last][left as usize];
        }
        let mut ret: i64 = 0;
        for j in 0..roll_max.len() {
            if last != j {
                ret += dfs(i - 1, j, roll_max[j] - 1, roll_max, cache) % mo;
            // 可用数字不够时 不再往下走
            } else if left > 0 {
                ret += dfs(i - 1, j, left - 1, roll_max, cache) % mo;
            }
        }
        cache[i as usize][last][left as usize] = ret;
        ret
    }
    let mut ret: i64 = 0;
    const mo: i64 = 1e9 as i64 + 7;
    let mut cache = vec![vec![vec![-1 as i64; 15]; roll_max.len()]; n as usize];
    for j in 0..roll_max.len() {
        ret += dfs(n - 1, j, roll_max[j] - 1, &roll_max, &mut cache) % mo;
    }
    (ret % mo) as i32
}