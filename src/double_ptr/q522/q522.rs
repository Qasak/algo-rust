impl Solution {
    // 结论：不是其余字符串的子序列的最长字符串，就是最长的特殊序列
    // 反证: 如果某个子序列是最长的特殊序列，那么添加任意字符，它也是最长的特殊序列，最长即其对应的字符串
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        // s 是否是t的子序列
        fn is_sub_seq(s: &[u8], t: &[u8]) -> bool {
            let mut i = 0;
            for &c in t {
                if s[i] == c {
                    i += 1;
                }
                if i == s.len() {
                    return true;
                }
            }
            return false;
        }

        let mut ans = -1;
        // 遍历s
        for (i, s) in strs.iter().enumerate() {
            // 判断s是否可更新ans
            if strs[i].len() as i32 <= ans {
                continue;
            }
            // 查询s是否是其他字符串的子序列
            let mut can_be_sub = false;
            for j in 0..strs.len() {
                // 可以为子序列，不更新
                // .bytes()返回一个字节迭代器（Bytes）
                // .as_bytes()返回一个字节切片（&[u8]）
                if i != j && is_sub_seq(strs[i].as_bytes(), strs[j].as_bytes()) {
                    can_be_sub = true;
                    break;
                }
            }
            if !can_be_sub {
                ans = strs[i].len() as i32;
            }
        }
        ans
    }
}