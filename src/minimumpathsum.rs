pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let y_size = grid.len();
        let x_size = grid[0].len();

        let mut temp = vec![vec![0; x_size]; y_size];

        for y in 0..y_size {
            for x in 0..x_size {
                temp[y][x] += grid[y][x];

                if x > 0 && y > 0 {
                    temp[y][x] += std::cmp::min(temp[y - 1][x], temp[y][x - 1]);
                } else if y > 0 {
                    temp[y][x] += temp[y - 1][x];
                } else if x > 0 {
                    temp[y][x] += temp[y][x - 1];
                }
            }
        }

        temp[y_size - 1][x_size - 1]
    }
}
