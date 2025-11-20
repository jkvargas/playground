use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let mut queue = VecDeque::new();

        for i in 0..rooms.len() {
            for j in 0..rooms[i].len() {
                if rooms[i][j] == 0 {
                    queue.push_back(vec![i as i32, j as i32]);
                }
            }
        }

        let directions = vec![vec![-1, 0], vec![1, 0], vec![0, 1], vec![0, -1]];

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();

            for dir in &directions {
                let row = pos[0] + dir[0];
                let col = pos[1] + dir[1];

                if row >= 0
                    && col >= 0
                    && (row as usize) < rooms.len()
                    && (col as usize) < rooms[0].len()
                    && rooms[row as usize][col as usize] == 2147483647
                {
                    rooms[row as usize][col as usize] = rooms[pos[0] as usize][pos[1] as usize] + 1;
                    queue.push_back(vec![row, col]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::walls_and_gates::Solution;

    #[test]
    fn test_one() {
        let mut vec: Vec<Vec<i32>> = vec![
            vec![2147483647, -1, 0, 2147483647],
            vec![2147483647, 2147483647, 2147483647, -1],
            vec![2147483647, -1, 2147483647, -1],
            vec![0, -1, 2147483647, 2147483647],
        ];
        Solution::walls_and_gates(&mut vec);
        assert_eq!(
            vec![
                vec![3, -1, 0, 1],
                vec![2, 2, 1, -1],
                vec![1, -1, 2, -1],
                vec![0, -1, 3, 4]
            ],
            vec
        );
    }
}
