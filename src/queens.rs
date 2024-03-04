use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut result = Vec::new();
        let mut diagonals = HashSet::new();
        let mut anti_diagonals = HashSet::new();
        let mut cols = HashSet::new();

        Self::backtrack(
            0,
            &mut diagonals,
            &mut anti_diagonals,
            &mut cols,
            &mut board,
            n,
            &mut result,
        );

        result
    }

    fn create_board(board: &Vec<Vec<char>>) -> Vec<String> {
        board
            .iter()
            .map(|x| x.into_iter().collect::<String>())
            .collect()
    }

    fn backtrack(
        row: i32,
        diagonals: &mut HashSet<i32>,
        anti_diagonals: &mut HashSet<i32>,
        cols: &mut HashSet<i32>,
        state: &mut Vec<Vec<char>>,
        n: i32,
        results: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            results.push(Self::create_board(state));
            return;
        }

        for col in 0..n {
            let cur_diagonal = row - col;
            let cur_anti_diagonal = row + col;

            if cols.contains(&col)
                || diagonals.contains(&cur_diagonal)
                || anti_diagonals.contains(&cur_anti_diagonal)
            {
                continue;
            }

            cols.insert(col);
            diagonals.insert(cur_diagonal);
            anti_diagonals.insert(cur_anti_diagonal);
            state[row as usize][col as usize] = 'Q';
            Self::backtrack(row + 1, diagonals, anti_diagonals, cols, state, n, results);

            cols.remove(&col);
            diagonals.remove(&cur_diagonal);
            anti_diagonals.remove(&cur_anti_diagonal);
            state[row as usize][col as usize] = '.';
        }
    }
}

#[cfg(test)]
mod test {
    use crate::queens::Solution;

    #[test]
    fn test_one() {
        assert_eq!(
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ],
            Solution::solve_n_queens(4)
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(vec![vec!["Q"]], Solution::solve_n_queens(1));
    }
}
