impl Solution {
    // n = s.len(), m = t.len()
    // i指向s, j指向t
    // 一个方案指t完全匹配，即j==m  返回1
    // i = n, j < m 则说明该方案匹配不上 返回0
    // 用s来匹配t
    // s[i]==t[j]时可以选择匹配或不匹配
    // 匹配：i += 1, j += 1
    // 不匹配：i += 1, j
    // s[i]!=t[j]时
    // i += 1, j
    pub fn num_distinct(s: String, t: String) -> i32 {
        let ss: Vec<char> = s.chars().collect();
        let tt: Vec<char> = t.chars().collect();
        let (n, m) = (ss.len(), tt.len());
        let mut cache = vec![vec![-1; m]; n];

        if n < m {
            return 0;
        }
        fn dfs(i: usize, j: usize, ss: &Vec<char>, tt: &Vec<char>, cache: &mut Vec<Vec<i32>>) -> i32{
            let (n, m) = (ss.len(), tt.len());

            // 包含i == n，j == m 的情况
            if j == m {
                return 1;
            }
            if i == n {
                return 0;
            }
            // 记忆化
            if cache[i][j] != -1 {
                return cache[i][j];
            }
            let mut cnt = dfs(i + 1, j, ss, tt, cache);
            if ss[i] == tt[j] {
                cnt += dfs(i + 1, j + 1, ss, tt, cache);
            } 
            cache[i][j] = cnt;
            cnt
        }
        dfs(0, 0, &ss, &tt, &mut cache)
    }


    pub fn num_distinct_dp(s: String, t: String) -> i32 {
        let ss: Vec<char> = s.chars().collect();
        let tt: Vec<char> = t.chars().collect();
        let (n, m) = (ss.len(), tt.len());
        if n < m {
            return 0;
        }
        let mut dp = vec![vec![0; m + 1]; n + 1];
        // 初始化'递归终点'
        for i in 0..=n {
            dp[i][m] = 1;
        }
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if ss[i] == tt[j] {
                    dp[i][j] = dp[i + 1][j + 1] + dp[i + 1][j];
                } else {
                    dp[i][j] = dp[i + 1][j];
                }
            }
        }
        dp[0][0]
    }
}