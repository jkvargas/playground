// https://leetcode.com/problems/keys-and-rooms/description/?envType=study-plan-v2&envId=leetcode-75

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut room_map = vec![false; rooms.len()];

        let mut queue = VecDeque::new();

        queue.push_back(0);

        while !queue.is_empty() {
            let room_index = queue.pop_front().unwrap();
            room_map[room_index] = true;

            for &i in &rooms[room_index] {
                if !room_map[i as usize] {
                    queue.push_back(i as usize);
                }
            }
        }

        room_map.iter().all(|x| *x)
    }
}

#[cfg(test)]
mod tests {
    use crate::keys_and_rooms::Solution;

    #[test]
    fn test_one() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];

        assert!(!Solution::can_visit_all_rooms(rooms));
    }
}
