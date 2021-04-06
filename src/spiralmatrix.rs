struct Solution;

enum Direction {
    Up,
    Down,
    Forward,
    Back,
}

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut direction = Direction::Forward;

        while matrix.len() > 0 && matrix.iter().any(|v| v.len() > 0) {
            match direction {
                Direction::Down => matrix
                    .iter_mut()
                    .for_each(|v| result.push(v.pop().unwrap())),
                Direction::Up => matrix
                    .iter_mut()
                    .rev()
                    .for_each(|v| result.push(v.remove(0))),
                Direction::Forward => result.append(&mut matrix.remove(0)),
                Direction::Back => matrix
                    .pop()
                    .unwrap()
                    .iter()
                    .rev()
                    .for_each(|i| result.push(*i)),
            }
            direction = Self::next_direction(direction);
        }

        result
    }

    pub fn spiral_order_1(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 {
            return Vec::new();
        }

        let mut result: Vec<i32> = Vec::new();

        let mut i = 0;
        let mut j = 0;

        let mut i_size = matrix.len();
        let mut j_size = matrix[0].len();

        while i < i_size && j < j_size {
            for j_pos in j..j_size {
                result.push(matrix[i][j_pos]);
            }

            for i_pos in i + 1..i_size {
                result.push(matrix[i_pos][j])
            }

            dbg!(&result);

            if i < i_size && j < j_size {
                for j_pos in (j..j_size).rev() {
                    result.push(matrix[i_size - 1][j_pos]);
                }

                for i_pos in (i..i_size).rev() {
                    result.push(matrix[i_pos][i])
                }
            }

            i += 1;
            i_size -= 1;
            j += 1;
            j_size -= 1;
        }

        result
    }

    fn next_direction(dir: Direction) -> Direction {
        match dir {
            Direction::Down => Direction::Back,
            Direction::Up => Direction::Forward,
            Direction::Forward => Direction::Down,
            Direction::Back => Direction::Up,
        }
    }

    fn move_through(
        direction: Direction,
        matrix: &Vec<Vec<i32>>,
        i: usize,
        j: usize,
        results: &mut Vec<i32>,
        visited: &mut Vec<Vec<bool>>,
        last_visited: &mut Vec<(i32, i32)>,
    ) -> bool {
        match direction {
            Direction::Up => {
                if i > 0 && !visited[i][j] {
                    last_visited[Direction::Up as usize] = (i as i32, j as i32);
                    visited[i][j] = true;
                    results.push(matrix[i][j]);
                    return Self::move_through(
                        Direction::Up,
                        matrix,
                        i - 1,
                        j,
                        results,
                        visited,
                        last_visited,
                    );
                } else {
                    let last_pos = last_visited[Direction::Forward as usize];

                    if (last_pos.0 == i as i32 && last_pos.1 == (j + 1) as i32)
                        || !Self::move_through(
                            Direction::Forward,
                            matrix,
                            i,
                            j + 1,
                            results,
                            visited,
                            last_visited,
                        )
                    {
                        return false;
                    }
                }
            }
            Direction::Down => {
                if i < matrix.len() && !visited[i][j] {
                    last_visited[Direction::Down as usize] = (i as i32, j as i32);
                    visited[i][j] = true;
                    results.push(matrix[i][j]);
                    return Self::move_through(
                        Direction::Down,
                        matrix,
                        i + 1,
                        j,
                        results,
                        visited,
                        last_visited,
                    );
                } else {
                    let last_pos = last_visited[Direction::Back as usize];

                    if (last_pos.0 == (i - 1) as i32 && last_pos.1 == (j - 1) as i32)
                        || !Self::move_through(
                            Direction::Back,
                            matrix,
                            i - 1,
                            j - 1,
                            results,
                            visited,
                            last_visited,
                        )
                    {
                        return false;
                    }
                }
            }
            Direction::Forward => {
                if j < matrix[0].len() && !visited[i][j] {
                    last_visited[Direction::Forward as usize] = (i as i32, j as i32);
                    visited[i][j] = true;
                    results.push(matrix[i][j]);
                    return Self::move_through(
                        Direction::Forward,
                        matrix,
                        i,
                        j + 1,
                        results,
                        visited,
                        last_visited,
                    );
                } else {
                    let last_pos = last_visited[Direction::Down as usize];

                    if (last_pos.0 == (i + 1) as i32 && last_pos.1 == (j - 1) as i32)
                        || !Self::move_through(
                            Direction::Down,
                            matrix,
                            i + 1,
                            j - 1,
                            results,
                            visited,
                            last_visited,
                        )
                    {
                        return false;
                    }
                }
            }
            Direction::Back => {
                if j > 0 && !visited[i][j] {
                    last_visited[Direction::Back as usize] = (i as i32, j as i32);
                    visited[i][j] = true;
                    results.push(matrix[i][j]);
                    return Self::move_through(
                        Direction::Back,
                        matrix,
                        i,
                        j - 1,
                        results,
                        visited,
                        last_visited,
                    );
                } else {
                    let last_pos = last_visited[Direction::Up as usize];
                    if (last_pos.0 == i as i32 && last_pos.1 == j as i32)
                        || !Self::move_through(
                            Direction::Up,
                            matrix,
                            i - 1,
                            j,
                            results,
                            visited,
                            last_visited,
                        )
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spiral_order_1() {
        let result = Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn spiral_order_2() {
        let result = Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ]);

        assert_eq!(result, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }
}
