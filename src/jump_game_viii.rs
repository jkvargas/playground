use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, costs: Vec<i32>) -> i64 {
        let mut map = HashMap::new();
        let low = j(&nums, &costs, 0, &mut map, false);
        let high = j(&nums, &costs, 0, &mut map, true);

        if low.is_some() && high.is_some() {
            return std::cmp::max(low.unwrap(), high.unwrap());
        }

        if low.is_some() {
            return low.unwrap();
        }

        high.unwrap()
    }
}

fn j(
    nums: &Vec<i32>,
    cost: &Vec<i32>,
    starting_pos: usize,
    memo: &mut HashMap<usize, i64>,
    bigger: bool,
) -> Option<i64> {
    if memo.contains_key(&starting_pos) {
        return Some(*memo.get(&starting_pos).unwrap());
    }

    if starting_pos == nums.len() - 1 {
        return Some(cost[starting_pos] as i64);
    }

    let follows_bigger = |i| if i > starting_pos { nums[i - 1] >= nums[starting_pos] } else { true } && nums[starting_pos] > nums[i];
    let follows_lower = |i| if i > starting_pos { nums[i - 1] < nums[starting_pos] } else { true } && nums[starting_pos] <= nums[i];

    let mut found_something = false;
    let mut min = i64::MAX;
    for i in starting_pos + 1..nums.len() {
        if (bigger && follows_bigger(i)) || follows_lower(i) {
            if let Some(nj) = j(nums, cost, i, memo, bigger) {
                min = std::cmp::min(min, cost[i] as i64 + nj);
                found_something = true;
            }
        }
    }

    if !found_something {
        return None;
    }

    memo.insert(starting_pos, min);
    Some(min)
}

#[cfg(test)]
mod tests {
    use crate::jump_game_viii::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            8,
            Solution::min_cost(vec![3, 2, 4, 4, 1], vec![3, 7, 6, 4, 2])
        );
    }
}
