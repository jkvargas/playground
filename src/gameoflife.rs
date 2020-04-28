pub struct Solution;

const ALIVE: i32 = 1;
const DEAD: i32 = 0;

impl Solution {
    pub fn game_of_life(mut board: &mut Vec<Vec<i32>>) {
        let i_size = board.len();
        let j_size = board[0].len();

        let mut killed : Vec<(usize, usize)> = Vec::new();
        let mut ressu : Vec<(usize, usize)> = Vec::new();

        for i in 0..i_size {
            for j in 0..j_size {
                let mut alive_neigbors = 0;
                if i + 1 < i_size {
                    if board[i + 1][j] == ALIVE {
                        alive_neigbors += 1;
                    }

                    if j + 1 < j_size && board[i + 1][j + 1] == ALIVE {
                        alive_neigbors += 1;
                    }

                    if j > 0 && board[i + 1][j - 1] == ALIVE {
                        alive_neigbors += 1;
                    }
                }

                if i > 0 {
                    if board[i - 1][j] == ALIVE {
                        alive_neigbors += 1;
                    }

                    if j + 1 < j_size && board[i - 1][j + 1] == ALIVE {
                        alive_neigbors += 1;
                    }

                    if j > 0 && board[i - 1][j - 1] == ALIVE {
                        alive_neigbors += 1;
                    }
                }

                if j + 1 < j_size && board[i][j + 1] == ALIVE {
                    alive_neigbors += 1;
                }

                if j > 0 && board[i][j - 1] == ALIVE {
                    alive_neigbors += 1;
                }

                if board[i][j] == ALIVE {
                    if alive_neigbors < 2 {
                        killed.push((i, j));
                        continue;
                    }

                    if alive_neigbors > 1 && alive_neigbors < 4 {
                        continue;
                    } else if alive_neigbors > 3 {
                        killed.push((i, j));
                    }
                } else {
                    if alive_neigbors == 3 {
                        ressu.push((i, j));
                    }
                }
            }
        }

        for i in killed {
            board[i.0][i.1] = DEAD;
        }

        for i in ressu {
            board[i.0][i.1] = ALIVE;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn game_of_life_1() {
        let mut vec = vec![vec![0, 1, 0],
                       vec![0, 0, 1],
                       vec![1, 1, 1],
                       vec![0, 0, 0]];

        let result = vec![vec![0, 0, 0],
                          vec![1, 0, 1],
                          vec![0, 1, 1],
                          vec![0, 1, 0]];

        Solution::game_of_life(&mut vec);

        assert_eq!(vec, result);
    }
}