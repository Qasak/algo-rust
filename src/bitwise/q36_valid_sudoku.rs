// TERRIBLE CODE
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    fn char_to_usize(c: char) -> usize {
        (c as u8 - b'0' ) as usize
    }
    let n = 9;
    for row in board.iter() {
        let mut vis = vec![false; n + 1];
        for &n in row {
            if n == '.' {continue;}
            if vis[char_to_usize(n)] {
                return false;
            }
            vis[char_to_usize(n)] = true;
        }
    }
    for i in 0..9 {
        let mut vis = vec![false; n + 1];
        for j in 0..9 {
            let n = board[j][i];
            if n == '.' {continue;}
            if vis[char_to_usize(n)] {
                return false;
            }
            vis[char_to_usize(n)] = true;
        }
    }
    let mut flag = true;
    (0..n).step_by(3).for_each(|i| {
        (0..n).step_by(3).for_each(|j| {
            let mut vis = vec![false; n + 1];
            for u in i..(i + 3) {
                for v in j..(j + 3) {
                    let n = board[u][v];
                    if n == '.' {continue;}
                    if vis[char_to_usize(n)] {
                        flag = false;
                    }
                    vis[char_to_usize(n)] = true;
                }
            }
        })
    });
    flag
}

// bitwise solution
