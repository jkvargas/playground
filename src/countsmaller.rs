struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        let mut indices: Vec<i32> = (0..n as i32).collect();

        Self::merge_sort(&mut indices, 0, n as i32, &mut result, &nums);

        result
    }

    fn merge_sort(
        indices: &mut Vec<i32>,
        left: i32,
        right: i32,
        result: &mut Vec<i32>,
        nums: &Vec<i32>,
    ) {
        if right - left <= 1 {
            return;
        }

        let mid = (left + right) / 2;
        Self::merge_sort(indices, left, mid, result, nums);
        Self::merge_sort(indices, mid, right, result, nums);
        Self::merge(indices, left, right, mid, result, nums);

        dbg!(&indices);
    }

    fn merge(
        indices: &mut Vec<i32>,
        left: i32,
        right: i32,
        mid: i32,
        result: &mut Vec<i32>,
        nums: &Vec<i32>,
    ) {
        let mut i = left;
        let mut j = mid;
        let mut temp = Vec::new();

        while i < mid && j < right {
            if nums[indices[i as usize] as usize] <= nums[indices[j as usize] as usize] {
                result[indices[i as usize] as usize] += j - mid;
                temp.push(indices[i as usize]);
                i += 1;
            } else {
                temp.push(indices[j as usize]);
                j += 1;
            }
        }

        while i < mid {
            result[indices[i as usize] as usize] += j - mid;
            temp.push(indices[i as usize]);
            i += 1;
        }

        while j < right {
            temp.push(indices[j as usize]);
            j += 1;
        }

        for k in left..right {
            indices[k as usize] = temp[(k - left) as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::countsmaller::Solution;

    #[test]
    fn test_one() {
        assert_eq!(vec![2, 1, 1, 0], Solution::count_smaller(vec![5, 2, 6, 1]));
    }
}
