pub struct Solution;

const FRESH_ORANGE: i32 = 1;
const ROTTEN_ORANGE: i32 = 2;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let len_x = grid.len();
        let len_y = grid[0].len();

        let mut rotten_positions : Vec<(usize, usize)> = Vec::new();
        let mut for_next_round_positions : Vec<(usize, usize)> = Vec::new();
        let mut clean_oranges = 0;
        let mut rounds = 0;

        for x in 0..len_x {
            for y in 0..len_y {
                if grid[x][y] == ROTTEN_ORANGE {
                    rotten_positions.push((x, y));
                } else if grid[x][y] == FRESH_ORANGE {
                    clean_oranges += 1;
                }
            }
        }

        while rotten_positions.len() > 0 {
            let mut add_round = false;

            while rotten_positions.len() > 0 {
                dbg!(&rotten_positions);

                let to_act = rotten_positions[0];
                rotten_positions.remove(0);

                let to_add = Self::stamp_around(&mut grid, to_act.0, to_act.1, len_x, len_y);

                if !add_round {
                    add_round = to_add.len() > 0;
                }

                clean_oranges -= to_add.len();

                for new_pos in to_add {
                    for_next_round_positions.push(new_pos);
                }
            }

            if add_round {
                rounds += 1;
            }

            rotten_positions.append(&mut for_next_round_positions);
        }

        if clean_oranges == 0 {
            rounds as i32
        } else {
            -1
        }
    }

    fn stamp_around(grid: &mut Vec<Vec<i32>>, posx: usize, posy: usize, lenx: usize, leny: usize) -> Vec<(usize, usize)> {
        let mut positions : Vec<(usize, usize)> = Vec::new();

        if posx + 1 < lenx && grid[posx + 1][posy] == FRESH_ORANGE {
            grid[posx + 1][posy] = ROTTEN_ORANGE;
            positions.push((posx + 1, posy));
        }

        if posx > 0 && grid[posx - 1][posy] == FRESH_ORANGE {
            grid[posx - 1][posy] = ROTTEN_ORANGE;
            positions.push((posx - 1, posy));
        }

        if posy + 1 < leny && grid[posx][posy + 1] == FRESH_ORANGE {
            grid[posx][posy + 1] = ROTTEN_ORANGE;
            positions.push((posx, posy + 1));
        }

        if posy > 0 && grid[posx][posy - 1] == FRESH_ORANGE {
            grid[posx][posy - 1] = ROTTEN_ORANGE;
            positions.push((posx, posy - 1));
        }

        positions
    }
}

// PODRE - BOA - BOA
// BOA   - BOA - NENHUMA
// NENHU - BOA - BOA
// 6 - BOAS
// 0.0 - PODRE

//      NENHUM
// NEN -- PODRE -- BOA POS X + 1, Y
//        BOA POS X, Y + 1
// ACHOU 2
// BOAS - 2 = 4
// TURNOS + 1

// PODRE - (PODRE) - BOA
// PODRE - BOA - NENHUM
// NEN   - BOA -- BOA

//       NENHUM
// PODRE - PODRE - BOA
//          BOA

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oranges_rotting_1() {
        assert_eq!(Solution::oranges_rotting(vec![vec![2,1,1],vec![1,1,0],vec![0,1,1]]), 4);
    }

    #[test]
    fn oranges_rotting_2() {
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
    }

    #[test]
    fn oranges_rotting_3() {
        assert_eq!(Solution::oranges_rotting(vec![vec![1],vec![2],vec![1],vec![2]]), 1);
    }
}