impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn dfs(i: usize, n: usize, ret: &mut Vec<String>, path: &mut Vec<char>) {
            if i == n {
                ret.push(path.iter().collect::<String>());
                return;
            }
            if !path[i].is_ascii_alphabetic() {
                dfs(i + 1, n, ret, path);
                return;
            }

            path[i] = path[i].to_ascii_lowercase();
            dfs(i + 1, n, ret, path);
            path[i] = path[i].to_ascii_uppercase();
            dfs(i + 1, n, ret, path);
        }
        let mut ret = vec![];
        dfs(0, s.len(), &mut ret, &mut s.chars().collect::<Vec<_>>());
        ret
    }
}