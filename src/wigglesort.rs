struct Solution;

impl Solution {
    pub fn wiggle_sort(mut nums: &mut Vec<i32>) {
        nums.sort();

        let mut counter = 2;

        while counter < nums.len() {
            let temp = nums[counter - 1];
            nums[counter - 1] = nums[counter];
            nums[counter] = temp;

            counter += 2;
        }

        dbg!(&nums);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiggle_sort_1() {
        let mut nums = vec![3, 5, 2, 1, 6, 4];

        Solution::wiggle_sort(&mut nums);

        assert_eq!(nums, vec![1, 3, 2, 5, 4, 6]);
    }
}
