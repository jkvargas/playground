struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..(n + 1) / 2 {
            for j in 0..n / 2 {
                let temp = matrix[n - 1 - j][i];
                matrix[n - 1 - j][i] = matrix[n - 1 - i][n - j - 1];
                matrix[n - 1 - i][n - j - 1] = matrix[j][n - 1 - i];
                matrix[j][n - 1 - i] = matrix[i][j];
                matrix[i][j] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rotatematrix::Solution;

    #[test]
    fn test_one() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        Solution::rotate(&mut matrix);

        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], matrix);
    }

    #[test]
    fn test_two() {
        let mut matrix = vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]];

        Solution::rotate(&mut matrix);

        assert_eq!(vec![vec![15, 13, 2, 5], vec![14, 3, 4, 1], vec![12, 6, 8, 9], vec![16, 7, 10, 11]], matrix);
    }
}