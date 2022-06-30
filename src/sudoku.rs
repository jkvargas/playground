use std::collections::HashSet;

//https://leetcode.com/problems/sudoku-solver/

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); board[0].len()];
        let mut lines: Vec<HashSet<char>> = vec![HashSet::new(); board.len()];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); board[0].len() / 3 * board.len() / 3];

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let box_number = i % 3 + j % 3;

                if vec[i][j] != '.' {
                    cols[j].insert(vec[i][j]);
                    lines[i].insert(vec[i][j]);

                }
            }
        }
    }
}