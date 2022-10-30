impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn dfs(i: usize, n: usize, ret: &mut Vec<String>, path: &mut Vec<u8>) {
            if i == n {
                let s = String::from_utf8(path.clone()).unwrap();
                ret.push(s);
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
        let mut path = s.bytes().collect::<Vec<u8>>();
        dfs(0, s.len(), &mut ret, &mut path);
        ret
    }
}