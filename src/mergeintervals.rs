use std::cmp::min;

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        if intervals.is_empty() {
            return result;
        }

        intervals.sort_by(|x, y| x[0].cmp(&y[0]));
        result.push(intervals[0].clone());

        if intervals.len() > 1 {
            for pos in 1..intervals.len() {
                let in_stack = result.pop().unwrap();

                if Self::overlaps(&in_stack, &intervals[pos]) {
                    result.push(vec![
                        in_stack[0],
                        std::cmp::max(in_stack[1], intervals[pos][1]),
                    ]);
                } else {
                    result.push(in_stack);
                    result.push(intervals[pos].clone());
                }
            }
        }

        result
    }

    fn overlaps(before: &Vec<i32>, after: &Vec<i32>) -> bool {
        let min_end = std::cmp::min(before[1], after[1]);

        if after[0] <= min_end {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn merge_2() {
        assert_eq!(
            Solution::merge(vec![vec![1, 7], vec![4, 5], vec![12, 14], vec![18, 19]]),
            vec![vec![1, 7], vec![12, 14], vec![18, 19]]
        );
    }

    #[test]
    fn merge_3() {
        assert_eq!(
            Solution::merge(vec![vec![1, 7], vec![4, 5], vec![12, 14], vec![18, 19]]),
            vec![vec![1, 7], vec![12, 14], vec![18, 19]]
        );
    }

    #[test]
    fn merge_4() {
        assert_eq!(
            Solution::merge(vec![vec![4, 9], vec![4, 5], vec![5, 12], vec![15, 21]]),
            vec![vec![4, 12], vec![15, 21]]
        );
    }

    #[test]
    fn merge_5() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }

    #[test]
    fn merge_6() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![0, 4]]),
            vec![vec![0, 4]]
        );
    }
}
