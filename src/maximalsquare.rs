const SOLID: char = '1';

struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let row_high = matrix.len();
        let col_high = matrix[0].len();

        let mut r_matrix = vec![vec![0; &col_high + 1]; &row_high + 1];
        let mut max_size = 0;

        for line in 1..=row_high {
            for col in 1..=col_high {
                if matrix[line - 1][col - 1] == SOLID {
                    r_matrix[line][col] = std::cmp::min(
                        r_matrix[line - 1][col],
                        std::cmp::min(r_matrix[line][col - 1], r_matrix[line - 1][col - 1]),
                    ) + 1;

                    max_size = std::cmp::max(max_size, r_matrix[line][col]);
                } else {
                    r_matrix[line][col] = 0;
                }
            }
        }

        max_size * max_size
    }
}

#[test]
pub fn test_one() {
    let matrix = vec![vec!['0', '1'], vec!['1', '0']];

    assert_eq!(3, Solution::maximal_square(matrix));
}
