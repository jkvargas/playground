use std::collections::HashSet;

//https://leetcode.com/problems/sudoku-solver/

struct Solution;

const BOX_SIZE: usize = 3;

struct Sudoku<'a> {
    board: &'a mut Vec<Vec<char>>,
    cols: Vec<HashSet<char>>,
    lines: Vec<HashSet<char>>,
    boxes: Vec<Vec<HashSet<char>>>,
    free_positions: Vec<Vec<usize>>,
}

impl<'a> Sudoku<'a> {
    pub fn new(board: &'a mut Vec<Vec<char>>) -> Self {
        let lines_len = board.len();
        let cols_len = board[0].len();
        let mut sudoku = Sudoku {
            free_positions: Vec::new(),
            board,
            lines: vec![HashSet::new(); lines_len],
            cols: vec![HashSet::new(); cols_len],
            boxes: vec![vec![HashSet::new(); cols_len / BOX_SIZE]; lines_len / BOX_SIZE],
        };

        for i in 0..sudoku.board.len() {
            for j in 0..sudoku.board[0].len() {
                if sudoku.board[i][j] != '.' {
                    sudoku.lines[i].insert(sudoku.board[i][j]);
                    sudoku.cols[j].insert(sudoku.board[i][j]);
                    sudoku.boxes[i / BOX_SIZE][j / BOX_SIZE].insert(sudoku.board[i][j]);
                } else {
                    sudoku.free_positions.push(vec![i, j]);
                }
            }
        }

        sudoku
    }

    pub fn solve(&mut self) {
        self.try_fill(0);
    }

    fn try_fill(&mut self, pos: usize) -> bool {
        if pos == self.free_positions.len() {
            return true;
        }

        for i in 1..10 {
            if self.can_place(i, pos) {
                self.put(i, pos);
                if self.try_fill(pos + 1) {
                    return true;
                }
                self.remove_number(pos);
            }
        }

        false
    }

    fn remove_number(&mut self, pos: usize) {
        let i = self.free_positions[pos][0];
        let j = self.free_positions[pos][1];
        self.lines[i].remove(&self.board[i][j]);
        self.cols[j].remove(&self.board[i][j]);
        self.boxes[i / BOX_SIZE][j / BOX_SIZE].remove(&self.board[i][j]);
        self.board[i][j] = '.';
    }

    fn can_place(&self, number: u32, pos: usize) -> bool {
        let letter = char::from_digit(number, 10).unwrap();
        let i = self.free_positions[pos][0];
        let j = self.free_positions[pos][1];
        !self.lines[i].contains(&letter)
            && !self.cols[j].contains(&letter)
            && !self.boxes[i / BOX_SIZE][j / BOX_SIZE].contains(&letter)
    }

    fn put(&mut self, number: u32, pos: usize) {
        let i = self.free_positions[pos][0];
        let j = self.free_positions[pos][1];
        self.board[i][j] = char::from_digit(number, 10).unwrap();
        self.lines[i].insert(self.board[i][j]);
        self.cols[j].insert(self.board[i][j]);
        self.boxes[i / BOX_SIZE][j / BOX_SIZE].insert(self.board[i][j]);
    }
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut sudoku = Sudoku::new(board);
        sudoku.solve();
    }
}
