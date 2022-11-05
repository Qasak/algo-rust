use std::collections::{HashMap, HashSet};

// naive backtrack: TLE
pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    fn dfs(word: &String, i: usize, x: usize, y: usize, board: &Vec<Vec<char>>, ret: &mut HashSet<String>, vis: &mut HashSet<(usize, usize)>) {
        if board[x][y] != word.chars().nth(i).unwrap() {
            return;
        }
        if i == word.len() - 1 {
            ret.insert(word.clone());
            return;
        }
        for (xx, yy) in [(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)] {
            if xx >= 0 && xx < board.len() && yy >= 0 && yy < board[0].len() && !vis.contains(&(xx, yy)) {
                vis.insert((xx, yy));
                dfs(word, i + 1, xx, yy, board, ret, vis);
                vis.remove(&(xx, yy));
            }
        }
    }
    let (n, m) = (board.len(), board[0].len());
    let mut vis = HashSet::new();
    let mut ret = HashSet::new();
    for word in words.iter() {
        for i in 0..n {
            for j in 0..m {
                vis.insert((i, j));
                dfs(word, 0, i, j, &board, &mut ret, &mut vis);
                vis.remove(&(i, j));
            }
        }
    }
    ret.into_iter().collect::<Vec<String>>()
}