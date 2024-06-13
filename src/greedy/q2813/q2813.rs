use std::collections::{HashSet, VecDeque};
impl Solution {
    // 子序列可以打乱顺序
    pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
        // 按profit从小到大排序
        items.sort_unstable_by_key(|item| -item[0]);
        let (mut res, mut profit) = (0 as i64, 0 as i64);
        // 用一个HashSet记录类别数，用一个VecDeque(stack)记录重复item
        let (mut cats, mut st) = (HashSet::new(), VecDeque::new());
        // enumerate()
        for (i, item) in items.iter().enumerate() {
            if i < k as usize {  
                profit += item[0] as i64;
                if !cats.contains(&item[1]) {
                    // insert()
                    cats.insert(item[1]);
                } else {
                    // 重复元素放到st, 堆顶为最小
                    st.push_back(item[0]);
                }
            } else if !cats.contains(&item[1]) && !st.is_empty()  {
                // 如果是新的类别则一定要替换堆顶的最小元素
                // 因为如果后面出现新类别，那么一定比当前元素更小, 这样会让总和的减少量最低
                // 最终答案并不一定采用替换后的, 有可能替换几个元素后，长度的增加导致比原res大
                profit += (item[0] - st.back().unwrap()) as i64;
                st.pop_back();
                cats.insert(item[1]);
            }
            // 每次都更新res
            res = res.max(profit + (cats.len() * cats.len()) as i64);
        }
        res
    }
}