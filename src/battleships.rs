pub struct Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        if board.is_empty() || board[0].is_empty() {
            return 0;
        }

        let size_x = board.len();
        let size_y = board[0].len();
        let mut ship_count = 0;

        for x in 0..size_x {
            for y in 0..size_y {
                if board[x][y] == 'X'
                    && !(x + 1 < size_x && board[x + 1][y] == 'X'
                        || y + 1 < size_y && board[x][y + 1] == 'X')
                {
                    ship_count += 1;
                }
            }
        }

        ship_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_node_1() {
        assert_eq!(
            Solution::count_battleships(vec![
                vec!['X', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
                vec!['.', '.', '.', 'X']
            ]),
            2
        );
    }
}
