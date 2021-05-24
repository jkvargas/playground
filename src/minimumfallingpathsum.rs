use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut memo = HashMap::new();

        let mut min = i32::MAX;

        for i in 0..matrix[0].len() {
            min = std::cmp::min(min, Self::f(0, i, &matrix, &mut memo))
        }

        min
    }

    fn f(
        start_i: usize,
        start_j: usize,
        matrix: &Vec<Vec<i32>>,
        memo: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if start_i >= matrix.len() {
            return 0;
        }

        if memo.contains_key(&(start_i, start_j)) {
            return *memo.get(&(start_i, start_j)).unwrap();
        }

        let minus_one = if start_j > 0 {
            Self::f(start_i + 1, start_j - 1, matrix, memo)
        } else {
            i32::MAX
        };

        let middle = Self::f(start_i + 1, start_j, matrix, memo);

        let plus_one = if start_j + 1 >= matrix[0].len() {
            i32::MAX
        } else {
            Self::f(start_i + 1, start_j + 1, matrix, memo)
        };

        let min =
            matrix[start_i][start_j] + std::cmp::min(minus_one, std::cmp::min(middle, plus_one));

        memo.insert((start_i, start_j), min);

        min
    }
}

#[test]
fn test_one() {
    assert_eq!(
        13,
        Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]])
    );
}
