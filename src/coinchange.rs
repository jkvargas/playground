pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut amountvec: Vec<i32> = vec![amount + 1; (amount + 1) as usize];
        amountvec[0] = 0;

        for i in 0..amount + 1 {
            for j in &coins {
                if *j <= i {
                    amountvec[i as usize] =
                        std::cmp::min(amountvec[i as usize], 1 + amountvec[(i - *j) as usize])
                }
            }
        }

        if amountvec[amount as usize] > amount {
            -1
        } else {
            amountvec[amount as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn coin_change_1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    pub fn coin_change_2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    pub fn coin_change_3() {
        assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
}
