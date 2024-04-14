use std::collections::{HashMap, VecDeque};

// https://leetcode.com/problems/time-taken-to-cross-the-door/description/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency
struct Solution;

pub enum DoorStatus {
    Entering,
    Existing,
    Unused
}

impl Solution {
    pub fn time_taken(arrival: Vec<i32>, state: Vec<i32>) -> Vec<i32> {
        let mut enter_line = VecDeque::new();
        let mut exit_line = VecDeque::new();
        let mut door_status = DoorStatus::Unused;
        let mut arrival_map = HashMap::new();
        let mut result = vec![0; arrival.len()];

        for (index, &value) in arrival.iter().enumerate() {
            arrival_map.entry(value as usize).or_insert(Vec::new()).push(index);
        }

        let mut time = 0;
        while time <= (2 * arrival.len()) {
            if let Some(people) = arrival_map.get(&time) {
                for i in people {
                    if state[*i] == 1 {
                        exit_line.push_back(*i);
                    } else {
                        enter_line.push_back(*i);
                    }
                }
            }

            let mut door_used_this_sec = DoorStatus::Unused;

            if !enter_line.is_empty() && !exit_line.is_empty() {
                match door_status {
                    DoorStatus::Entering => {
                        let front = enter_line.pop_front().unwrap();
                        result[front] = time as i32;
                        door_used_this_sec = DoorStatus::Entering;
                    }
                    DoorStatus::Existing | DoorStatus::Unused => {
                        let front = exit_line.pop_front().unwrap();
                        result[front] = time as i32;
                        door_used_this_sec = DoorStatus::Existing;
                    }
                }

                door_status = door_used_this_sec;
                time += 1;
                continue;
            }

            if !enter_line.is_empty() || !exit_line.is_empty() {
                let front = if enter_line.is_empty() {
                    door_used_this_sec = DoorStatus::Existing;
                    exit_line.pop_front().unwrap()
                } else {
                    door_used_this_sec = DoorStatus::Entering;
                    enter_line.pop_front().unwrap()
                };

                result[front] = time as i32;
            }

            door_status = door_used_this_sec;
            time += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::time_to_cross_the_door::Solution;

    #[test]
    fn test_one() {
        assert_eq!(vec![0, 3, 1, 2, 4], Solution::time_taken(vec![0, 1, 1, 2, 4], vec![0, 1, 0, 0, 1]));
    }
}
