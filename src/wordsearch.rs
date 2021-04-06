struct Solution;

impl Solution {
    // O(board.len() * board[0].len() * 4 ^ word.len())
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let i_len = board.len();
        let j_len = board[0].len();

        let letters = word.chars().into_iter().collect::<Vec<char>>();
        let mut letter_pos = 0;

        for i in 0..i_len {
            for j in 0..j_len {
                if Self::check_around(&letters, 0, &mut board, i as i32, j as i32) {
                    return true;
                }
            }
        }

        false
    }

    fn check_around(
        letters: &Vec<char>,
        letter_pos: usize,
        board: &mut Vec<Vec<char>>,
        i: i32,
        j: i32,
    ) -> bool {
        if letter_pos == letters.len() {
            return true;
        }

        if i < 0
            || j < 0
            || i as usize >= board.len()
            || j as usize >= board[0].len()
            || board[i as usize][j as usize] != letters[letter_pos]
        {
            return false;
        }

        board[i as usize][j as usize] == '#';

        if Self::check_around(letters, letter_pos + 1, board, i + 1, j)
            || Self::check_around(letters, letter_pos + 1, board, i - 1, j)
            || Self::check_around(letters, letter_pos + 1, board, i, j + 1)
            || Self::check_around(letters, letter_pos + 1, board, i, j - 1)
        {
            return true;
        }

        board[i as usize][j as usize] = letters[letter_pos];

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exist_1() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            ),
            true
        );
    }

    #[test]
    fn exist_2() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_string()
            ),
            true
        );
    }

    #[test]
    fn exist_3() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_string()
            ),
            false
        );
    }
}
