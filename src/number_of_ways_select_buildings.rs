// https://leetcode.com/problems/number-of-ways-to-select-buildings/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency

// Step1. Count total number if zeros and ones in string. (count0 and count1)
// Step2. Iterate the string from left and keep count of zeroes and ones in till current index.(cur0 and cur1)
// Step3. There is only two case is possible 101 and 010.
// If you encounter 1 then check for case 010
// Add total no of combinations of 010 formed by taking currrent 1 as centre i.e
// = (total no of 0s on left) * (total no of 0s on right) = cur0 * (count0-cur0)
// If you encounter 0 then check for case 101
// Add total no of combinations of 101 formed by taking currrent 0 as centre i.e
// = (total no of 1s on left) * (total no of 1s on right) = cur1 * (count1-cur1)
// return final ans.

struct Solution;

impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        let letters = s.chars().collect::<Vec<char>>();
        let mut count0 = 0;
        let mut count1 = 0;

        for i in 0..letters.len() {
            if letters[i] == '0' {
                count0 += 1;
            } else {
                count1 += 1;
            }
        }

        let mut cur1 = 0;
        let mut ans = 0;
        let mut cur0 = 0;

        for i in 0..letters.len() {
            if letters[i] == '1' {
                cur1 += 1;
                ans += cur0 * (count0 - cur0);
            } else {
                cur0 += 1;
                ans += cur1 * (count1 - cur1);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::number_of_ways_select_buildings::Solution;

    #[test]
    fn test_one() {
        assert_eq!(6, Solution::number_of_ways("001101".to_string()));
    }
}
