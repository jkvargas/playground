struct TicTacToe {
    n: usize,
    board: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TicTacToe {
    /** Initialize your data structure here. */
    fn new(n: i32) -> Self {
        TicTacToe {
            n: n as usize,
            board: vec![vec![0; n as usize]; n as usize],
        }
    }

    /** Player {player} makes a move at ({row}, {col}).
    @param row The row of the board.
    @param col The column of the board.
    @param player The player, can be either 1 or 2.
    @return The current winning condition, can be either:
            0: No one wins.
            1: Player 1 wins.
            2: Player 2 wins. */
    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let c_row = row as usize;
        let c_col = col as usize;

        if self.board[c_row][c_col] == 0 {
            self.board[c_row][c_col] = player;
        }

        dbg!(&self.board);

        if self.check_horizontal(c_row, c_col, player)
            || self.check_vertical(c_row, c_col, player)
            || self.check_main_diagonal(c_row, c_col, player)
            || self.check_opposite_diagonal(c_row, c_col, player)
        {
            return player;
        }

        0
    }

    fn check_horizontal(&self, row: usize, col: usize, player: i32) -> bool {
        for i in 0..self.n {
            if i == col {
                continue;
            }

            if self.board[row][i] != player {
                return false;
            }
        }

        true
    }

    fn check_vertical(&self, row: usize, col: usize, player: i32) -> bool {
        for i in 0..self.n {
            if i == row {
                continue;
            }

            if self.board[i][col] != player {
                return false;
            }
        }

        true
    }

    fn check_opposite_diagonal(&self, row: usize, col: usize, player: i32) -> bool {
        if row + col == self.n - 1 {
            for x in 0..self.n {
                let y = self.n - 1 - x;

                if row == x && col == y {
                    continue;
                }

                if self.board[x][y] != player {
                    return false;
                }
            }

            return true;
        }

        false
    }

    fn check_main_diagonal(&self, row: usize, col: usize, player: i32) -> bool {
        if row == col {
            for i in 0..self.n {
                if i == col {
                    continue;
                }

                if self.board[i][i] != player {
                    return false;
                }
            }

            return true;
        }

        false
    }
}

#[test]
fn game_1() {
    let mut obj = TicTacToe::new(3);

    assert_eq!(0, obj.make_a_move(0, 0, 1));
    assert_eq!(0, obj.make_a_move(0, 2, 2));
    assert_eq!(0, obj.make_a_move(2, 2, 1));
    assert_eq!(0, obj.make_a_move(1, 1, 2));
    assert_eq!(0, obj.make_a_move(2, 0, 1));
    assert_eq!(0, obj.make_a_move(1, 0, 2));
    assert_eq!(1, obj.make_a_move(2, 1, 1));
}

#[test]
fn game_2() {
    let mut obj = TicTacToe::new(2);

    assert_eq!(0, obj.make_a_move(0, 1, 1));
    assert_eq!(0, obj.make_a_move(1, 1, 2));
    assert_eq!(1, obj.make_a_move(1, 0, 1));
}
