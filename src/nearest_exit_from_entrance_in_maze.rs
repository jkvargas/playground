// https://leetcode.com/problems/nearest-exit-from-entrance-in-maze/description/?envType=study-plan-v2&envId=leetcode-75

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let steps = vec![vec![0, 1], vec![0, -1], vec![1, 0], vec![-1, 0]];
        let x_bound = maze.len() as i32 - 1;
        let y_bound = maze[0].len() as i32 - 1;

        let mut min_to_leave = i32::MAX;
        let mut queue = VecDeque::new();

        queue.push_back((entrance[0], entrance[1], 0));
        maze[entrance[0] as usize][entrance[1] as usize] = '+';

        while !queue.is_empty() {
            let (x, y, jumps) = queue.pop_front().unwrap();
            for step in &steps {
                let pos = (x + step[0], y + step[1], jumps + 1);
                if (pos.0 < 0 || pos.0 == x_bound + 1 || pos.1 < 0 || pos.1 == y_bound + 1) || maze[pos.0 as usize][pos.1 as usize] == '+' { continue; }

                if pos.0 == x_bound || pos.1 == y_bound || pos.0 == 0 || pos.1 == 0 {
                    min_to_leave = std::cmp::min(min_to_leave, pos.2);
                }

                maze[pos.0 as usize][pos.1 as usize] = '+';
                queue.push_back(pos);
            }
        }

        if min_to_leave == i32::MAX {
            -1
        } else {
            min_to_leave
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // '+','*','+','+','+','+','+'
    // '+','*','+','*','*','*','+'
    // '+','*','+','*','+','*','+'
    // '+','*','*','*','+','*','+'
    // '+','+','+','+','+','*','+'
    #[test]
    fn example_2() {
        assert_eq!(12, Solution::nearest_exit(vec![vec!['+','.','+','+','+','+','+'],vec!['+','.','+','.','.','.','+'],vec!['+','.','+','.','+','.','+'],vec!['+','.','.','.','+','.','+'],vec!['+','+','+','+','+','.','+']], vec![0, 1]))
    }

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::nearest_exit(vec![vec!['+','+','.','+'],vec!['.','.','.','+'],vec!['+','+','+','.']], vec![1, 2]));
    }
}