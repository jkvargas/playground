use std::collections::{HashMap, HashSet, VecDeque};

//https://leetcode.com/problems/minimum-knight-moves/

struct Solution;

impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        let mut directions = vec![
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
            (-1, 2),
        ];
        let mut steps = 0;
        let mut dequeue = VecDeque::new();
        let mut hash_set: HashSet<(i32, i32)> = HashSet::new();

        dequeue.push_back((0, 0));

        while dequeue.len() > 0 {
            let current_level_size = dequeue.len();
            for i in 0..current_level_size {
                let (pos_x, pos_y) = dequeue.pop_front().unwrap();
                if pos_x == x && pos_y == y {
                    return steps;
                }

                for (dir_x, dir_y) in &directions {
                    let new_pos = (dir_x + pos_x, dir_y + pos_y);

                    if hash_set.get(&new_pos).is_none() {
                        hash_set.insert(new_pos);
                        dequeue.push_back(new_pos);
                    }
                }
            }

            steps += 1;
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use crate::minimumknightmoves::Solution;

    #[test]
    pub fn test_one() {
        assert_eq!(1, Solution::min_knight_moves(2, 1));
    }
}
