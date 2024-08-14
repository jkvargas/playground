// https://leetcode.com/problems/unique-paths-ii/description/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut memo = HashMap::new();
        let result = up(&obstacle_grid, 0, 0, &mut memo);

        result
    }
}

fn up(grid: &Vec<Vec<i32>>, pos_i: usize, pos_j: usize, memo: &mut HashMap<(usize, usize), i32>) -> i32 {
    if pos_i >= grid.len() || pos_j >= grid[0].len() || grid[pos_i][pos_j] == 1 {
        return 0;
    }

    if pos_i == grid.len() - 1 && pos_j == grid[0].len() - 1 {
        return 1;
    }

    if let Some(res) = memo.get(&(pos_i, pos_j)) {
        return *res;
    }

    let result = up(grid, pos_i + 1, pos_j, memo) + up(grid, pos_i, pos_j + 1, memo);

    memo.insert((pos_i, pos_j), result);

    result
}

#[cfg(test)]
mod tests {
    use crate::unique_paths_ii::Solution;

    #[test]
    fn test_one() {
        assert_eq!(2, Solution::unique_paths_with_obstacles(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]))
    }
}