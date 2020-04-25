struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut vec : Vec<i32> = vec![1; nums.len()];

        Self::go_through(&mut vec, &nums, 0, 1);

        vec
    }

    pub fn go_through(result: &mut Vec<i32>, vec: &Vec<i32>, i: usize, left: i32) -> i32 {
        if i == vec.len() - 1 {
            result[i] = left;
            return vec[i];
        }

        let right = Self::go_through(result, &vec, i + 1, left * vec[i]);

        result[i] = left * right;

        vec[i] * right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_except_self_1() {
        let result = Solution::product_except_self(vec![1,2,3,4]);

        assert_eq!(vec![24,12,8,6], result);
    }
}