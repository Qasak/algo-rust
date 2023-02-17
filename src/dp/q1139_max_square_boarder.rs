pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();

    let mut left = vec![vec![0; m + 1]; n + 1];
    let mut up = vec![vec![0; m + 1]; n + 1];
    let mut max_border = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 1 {
                left[i + 1][j + 1] = left[i + 1][j] + 1;
                up[i + 1][j + 1] = up[i][j + 1] + 1;

                let mut border = left[i + 1][j + 1].min(up[i + 1][j + 1]);
                while left[i + 1 - border + 1][j + 1] < border || up[i + 1][j + 1 - border + 1] < border {
                    border -= 1;
                }

                max_border = max_border.max(border);
            }
        }
    }

    (max_border * max_border) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_1_bordered_square() {
        let grid = vec![            
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1],
        ];
        assert_eq!(largest1_bordered_square(grid), 9);

        let grid = vec![           
             vec![1, 1],
            vec![1, 1],
        ];
        assert_eq!(largest1_bordered_square(grid), 4);

        let grid = vec![            
            vec![1, 0],
            vec![1, 1],
        ];
        assert_eq!(largest1_bordered_square(grid), 1);

        let grid = vec![            
            vec![1, 1],
            vec![0, 0],
        ];
        assert_eq!(largest1_bordered_square(grid), 1);

        let grid = vec![            
            vec![1],
        ];
        assert_eq!(largest1_bordered_square(grid), 1);

        let grid = vec![            
            vec![0],
        ];
        assert_eq!(largest1_bordered_square(grid), 0);

        let grid = vec![            
            vec![1, 1, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(largest1_bordered_square(grid), 9);
    }
}