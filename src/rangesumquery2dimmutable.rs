struct NumMatrix {
    dp: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return NumMatrix {
                dp: vec![],
            };
        };

        let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                dp[i][j + 1] = matrix[i][j] + dp[i][j];
            }
        }

        NumMatrix {
            dp
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;
        for row in row1..=row2 {
            sum += self.dp[row as usize][(col2 + 1) as usize] - self.dp[row as usize][col1 as usize];
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use crate::rangesumquery2dimmutable::NumMatrix;

    #[test]
    fn test_one() {
        let num_matrix = NumMatrix::new(vec![vec![3, 0, 1, 4, 2], vec![5, 6, 3, 2, 1], vec![1, 2, 0, 1, 5], vec![4, 1, 0, 1, 7], vec![1, 0, 3, 0, 5]]);

        assert_eq!(8, num_matrix.sum_region(2, 1, 4, 3));
        assert_eq!(11, num_matrix.sum_region(1, 1, 2, 2));
        assert_eq!(12, num_matrix.sum_region(1, 2, 2, 4));
    }
}