pub struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut index = 0;
        let mut val = a[index];

        for i in 1..a.len() {
            if a[i] > val {
                val = a[i];
                index = i;
            }
        }

        index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peak_index_in_mountain_array_1() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    }

    #[test]
    fn peak_index_in_mountain_array_2() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
    }
}