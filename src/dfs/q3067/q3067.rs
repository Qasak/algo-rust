impl Solution {
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        // 一.建图/树：邻接表，保存从节点出发的所有边，用元组表示
        let n = edges.len() + 1;
        // 生成vec: Vec::new()
        let mut graph = vec![Vec::new(); n];
        // 1.iter()是一个对集合中元素的不可变借用：&T
        // 2.集合不被消费
        for edge in edges.iter() {
            let (a, b, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[a].push((b, cost));
            graph[b].push((a, cost));
        }
        // 二.dfs: 子树可连接到根的节点数
        // 1.参数: 图graph &Vec<Vec<(usize, i32)>>, 当前节点u, 根节点root(防止走回头路), curr: 当前节点到根的距离, signalSpeed 
        fn dfs(graph: &Vec<Vec<(usize, i32)>>, u: usize, root: usize, curr: i32, signal_speed: i32) -> i32 {
            let mut res = 0;
            if curr == 0 {
                res += 1;
            }
            // 遍历u的下一层节点v, root -- u -- (v1, v2, v3)
            // 遍历的时候一定会经过root，需要排除
            for &(v, cost) in graph[u].iter() {
                // 不能往回走
                if v != root {
                    res += dfs(graph, v, u, (curr + cost) % signal_speed, signal_speed);
                }
            }
            res
        }
        // 三.计数连接对数:
        let mut ret = vec![0; n];
        for root in 0..n {
            let mut pre = 0;
            for &(v, cost) in graph[root].iter() {
                let cnt = dfs(&graph, v, root, cost % signal_speed, signal_speed);
                ret[root] += pre * cnt;
                pre += cnt;
            }
        }
        ret
    }
}