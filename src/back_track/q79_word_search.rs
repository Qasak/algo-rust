use std::collections::{VecDeque, HashSet};
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    fn dfs(board: &Vec<Vec<char>>, word: &Vec<char>, i: usize, j: usize, idx: usize, vis: &mut HashSet<(usize, usize)>) -> bool {
        if word.len() == idx {
            return true;
        }
        let (i, j, n, m, mut flag) = (i as i32, j as i32, board.len() as i32, board[0].len() as i32, false);
        for (ii, jj) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
            if ii >= 0 && ii < n && jj >= 0 && jj < m {
                let (ii, jj) = (ii as usize, jj as usize);
                if board[ii][jj] == word[idx] && !vis.contains(&(ii, jj)){
                    vis.insert((ii, jj));
                    flag = flag||dfs(board, word, ii, jj, idx + 1, vis);
                    vis.remove(&(ii, jj));
                }
            }
        }
        flag
    }
    let (n, m, mut vis, cs) = (board.len(), board[0].len(), HashSet::new(), word.chars().collect::<Vec<char>>());
    for i in 0..n {
        for j in 0..m {
            if board[i][j] == cs[0] {
                vis.insert((i, j));
                if dfs(&board, &cs, i, j, 1, &mut vis) {
                    return true;
                }
                vis.remove(&(i, j));
            }
        }
    }
    false
}