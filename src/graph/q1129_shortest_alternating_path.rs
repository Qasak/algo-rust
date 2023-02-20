// naive bfs
pub fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
) -> Vec<i32> {
    use std::collections::{HashSet, VecDeque};
    fn build_graph(
        n: usize,
        red_edges: &Vec<Vec<i32>>,
        blue_edges: &Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut graph = vec![vec![-(n as i32); n as usize]; n as usize];
        for re in red_edges {
            graph[re[0] as usize][re[1] as usize] = 1;
        }
        for be in blue_edges {
            graph[be[0] as usize][be[1] as usize] = if graph[be[0] as usize][be[1] as usize] == 1 {
                0
            } else {
                -1
            };
        }
        graph
    }
    let (graph, mut queue, mut visited, mut answer, mut len) = (
        build_graph(n as usize, &red_edges, &blue_edges),
        VecDeque::new(),
        HashSet::new(),
        vec![i32::MAX; n as usize],
        0,
    );
    queue.push_back((0, 1));
    queue.push_back((0, -1));
    answer[0] = 0;
    while !queue.is_empty() {
        len += 1;
        let size = queue.len();
        for _ in 0..size {
            let curr = queue.pop_front().unwrap();
            let (node, opposite_color) = (curr.0, -curr.1);
            for i in 0..n as usize {
                if graph[node][i] != opposite_color && graph[node][i] != 0 {
                    continue;
                }
                if !visited.insert((i, opposite_color)) {
                    continue;
                }
                queue.push_back((i as usize, opposite_color));
                answer[i] = answer[i].min(len);
            }
        }
    }
    for i in 1..n as usize {
        answer[i] = if answer[i] == i32::MAX { -1 } else { answer[i] };
    }
    answer
}
