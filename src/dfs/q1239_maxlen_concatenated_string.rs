// leetcode 2022-10-24 EN
pub fn max_length(arr: Vec<String>) -> i32 {
    // max_length from index i
    fn dfs(i: usize, cur: String,  arr: &Vec<String>) -> i32 {
        if i == arr.len() {
            return cur.len() as i32;
        }
        let mut a = 0;
        let mut b = 0;
        // to i , choose or not choose
        let s = cur.clone() + arr[i].as_str();
        if check(&s) {
            a = dfs(i + 1, s, arr);
        }
        b = dfs(i + 1, cur.clone(), arr);
        b.max(a)
    }

    fn check(cur: &String) -> bool {
        let mut cnt = vec![0; 26];
        for ch in cur.bytes() {
            let idx = (ch - b'a') as usize;
            cnt[idx] += 1;
            if cnt[idx] > 1 {
                return false;
            }
        }
        true
    }
    dfs(0, "".to_string(), &arr)
}
