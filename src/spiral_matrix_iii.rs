// https://leetcode.com/problems/spiral-matrix-iii/editorial/?envType=daily-question&envId=2024-08-08

struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(
        rows: i32,
        cols: i32,
        mut r_start: i32,
        mut c_start: i32,
    ) -> Vec<Vec<i32>> {
        let directions = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
        let mut traversed = Vec::new();
        let mut direction = 0;
        let mut step = 1;

        loop {
            if rows as usize * cols as usize <= traversed.len() {
                break;
            }

            for i in 0..2 {
                for j in 0..step {
                    if r_start > 0 && r_start < rows && c_start >= 0 && c_start < cols {
                        traversed.push(vec![r_start, c_start]);
                    }

                    r_start += directions[direction][0];
                    c_start += directions[direction][1];
                }
                direction = (direction + 1) % 4;
            }

            step += 1;
        }

        traversed
    }
}

#[test]
fn test_one() {
    assert_eq!(
        vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]],
        Solution::spiral_matrix_iii(1, 4, 0, 0)
    );
}
