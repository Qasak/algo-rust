pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (n, m) = (matrix.len(), matrix[0].len());
    let (mut up, mut right, mut down, mut left) = (0, m, n, 0);
    let mut ret = Vec::with_capacity(n * m);
    let mut round = 0;
    while up < down && left < right {
        match round {
            0 => {
                for i in left..right {
                    ret.push(matrix[up][i]);
                }
                up += 1;
            }
            1 => {
                for i in up..down {
                    ret.push(matrix[i][right - 1]);
                }
                right -= 1;
            }
            2 => {
                for i in (left..right).rev() {
                    ret.push(matrix[down - 1][i]);
                }
                down -= 1;
            }
            3 => {
                for i in (up..down).rev() {
                    ret.push(matrix[i][left]);
                }
                left += 1;
            }
            _ => {}
        }
        round += 1;
        round %= 4;
    }
    ret
}
