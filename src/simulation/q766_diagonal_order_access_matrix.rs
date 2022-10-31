pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
    let n = matrix.len();
    let m = matrix[0].len();
    for i in 0..n {
        let (mut x, mut y) = (i, 0);
        let v = matrix[i][0];
        while x < n && y < m {
            if v != matrix[x][y] {
                return false;
            }
            x += 1; y += 1;
        }
    }
    for i in 0..m {
        let (mut x, mut y) = (0, i);
        let v = matrix[0][i];
        while x < n && y < m {
            if v != matrix[x][y] {
                return false;
            }
            x += 1; y += 1;
        }
    }
    true
}