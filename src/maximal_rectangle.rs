struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut dp = vec![vec![0; m + 1]; n];
        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == '1' {
                    dp[i][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i][j + 1] = 0;
                }

                ans = ans.max(dp[i][j + 1]);
            }
        }

        for i in 0..n {
            let mut cur = vec![i32::MAX; m + 1];
            for j in i..n {
                for k in 1..(m + 1) {
                    cur[k] = cur[k].min(dp[j][k]);
                    ans = ans.max(cur[k] * (j - i + 1) as i32);
                }
            }
        }

        ans
    }
}

struct BestSolution;

impl BestSolution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let cnt_row = matrix.len();
        let cnt_col = matrix[0].len();

        let mut heights: Vec<i32> = vec![0; cnt_col];
        let mut max_area = 0;

        for i in 0..cnt_row {
            for j in 0..cnt_col {
                if matrix[i][j] == '1' {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }

            let cur_max_area = Self::get_max_rectangle(&heights);
            max_area = std::cmp::max(max_area, cur_max_area);
        }

        max_area
    }

    fn get_max_rectangle(heights: &Vec<i32>) -> i32 {
        let mut heights = heights.clone();
        heights.push(0);
        heights.insert(0, 0);
        let mut stack: Vec<usize> = vec![0];
        let mut max_area = 0;
        for i in 1..heights.len() {
            while heights[i] < heights[*stack.last().unwrap()] {
                let idx = stack.pop().unwrap();
                let h = heights[idx];
                let w = (i - *stack.last().unwrap() - 1) as i32;
                max_area = std::cmp::max(max_area, h * w);
            }
            stack.push(i);
        }

        max_area
    }
}

#[cfg(test)]
mod test {
    use crate::maximal_rectangle::Solution;

    #[test]
    fn test_one() {
        assert_eq!(
            6,
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ])
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(1, Solution::maximal_rectangle(vec![vec!['1']]));
    }

    #[test]
    fn test_three() {
        assert_eq!(
            3,
            Solution::maximal_rectangle(vec![
                vec!['0', '0', '0'],
                vec!['0', '0', '0'],
                vec!['1', '1', '1']
            ])
        );
    }
}
