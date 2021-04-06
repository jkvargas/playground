pub struct Solution;

impl Solution {
    pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
        stones.sort();

        while stones.len() > 1 {
            let a = stones.get(stones.len() - 1).unwrap();
            let b = stones.get(stones.len() - 2).unwrap();

            let sub = a - b;
            stones.remove(stones.len() - 1);
            stones.remove(stones.len() - 1);

            if sub > 0 {
                let pos = stones.binary_search(&sub).unwrap_or_else(|e| e);
                stones.insert(pos, sub);
            }
        }

        if stones.is_empty() {
            return 0;
        }

        stones.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_stone_weight_1() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    fn last_stone_weight_2() {
        assert_eq!(Solution::last_stone_weight(vec![9, 3, 2, 10]), 2);
    }
}
