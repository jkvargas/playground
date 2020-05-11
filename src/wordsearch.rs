struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let i_len = board.len();
        let j_len = board[0].len();

        let letters = word.chars().into_iter().collect::<Vec::<char>>();
        let mut letter_pos = 0;

        for i in 0..i_len {
            for j in 0..j_len {
                if board[i][j] == letters[letter_pos] {
                    if Self::check_around(&letters, letter_pos + 1, &board, i, j) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn check_around(letters: &Vec<char>,
                    letter_pos: usize,
                    board: &Vec<Vec<char>>,
                    i: usize,
                    j: usize) -> bool {
        if letter_pos >= letters.len() {
            return true;
        }
        let looking_for = letters[letter_pos];

        let mut new_i = Self::add(1, i, board.len());
        if board[new_i][j] == looking_for {
            return Self::check_around(letters, letter_pos + 1, board, new_i, j);
        }

        new_i = Self::add(-1, i, board.len());
        if board[new_i][j] == looking_for {
            return Self::check_around(letters, letter_pos + 1, board, new_i, j);
        }

        let mut new_j = Self::add(1, j, board[0].len());
        if board[i][new_j] == looking_for {
            return Self::check_around(letters, letter_pos + 1, board, i, new_j);
        }

        new_j = Self::add(-1, j, board[0].len());
        if board[i][new_j] == looking_for {
            return Self::check_around(letters, letter_pos + 1, board, i, new_j);
        }

        return false;
    }

    fn add(val: i32, target: usize, board_len: usize) -> usize {
        if val > 0 {
            if target + (val as usize) > board_len {
                return 0;
            } else {
                return target + (val as usize);
            }
        } else {
            if (target as i32) + val < 0 {
                return board_len - 1;
            } else {
                return target - (val as usize);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exist_1() {
        assert_eq!(Solution::exist(vec![
            vec!['A','B','C','E'],
            vec!['A','B','C','E'],
            vec!['A','D','E','E']], "ABCCED".to_string()), true);
    }

    #[test]
    fn exist_2() {
        assert_eq!(Solution::exist(vec![
            vec!['A','B','C','E'],
            vec!['A','B','C','E'],
            vec!['A','D','E','E']], "SEE".to_string()), true);
    }

    #[test]
    fn exist_3() {
        assert_eq!(Solution::exist(vec![
            vec!['A','B','C','E'],
            vec!['A','B','C','E'],
            vec!['A','D','E','E']], "ABCB".to_string()), false);
    }
}