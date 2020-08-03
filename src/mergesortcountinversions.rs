pub struct Solution;

impl Solution {
    pub fn count_inversions(mut arr: Vec<i32>) -> i32 {
        let size = arr.len();
        Self::merge_sort(&mut arr, 0, size - 1)
    }

    fn merge(arr: &mut Vec<i32>, l: usize, m: usize, r: usize) -> i32 {
        let mut swap = 0;
        let n1 = m - l + 1;
        let n2 = r - m;

        let mut left_array = vec![0i32; n1];
        let mut right_array = vec![0i32; n2];

        for i in 0..n1 {
            left_array[i] = arr[l + i];
        }

        for i in 0..n2 {
            right_array[i] = arr[m + i + 1];
        }

        let mut i = 0;
        let mut k = 0;
        let mut j = l;

        while i < n1 && j < n2 {
            if left_array[i] <= right_array[j] {
                arr[k] = left_array[i];
                i += 1;
            } else {
                arr[k] = right_array[j];
                j += 1;

                let to_add = m - l + 1 - i;
                swap += to_add;
            }
            k += 1;
        }

        while i < n1 {
            arr[k] = left_array[i];
            i += 1;
            k += 1;
        }

        while j < n2 {
            arr[k] = right_array[j];
            j += 1;
            k += 1;
        }

        swap as i32
    }

    fn merge_sort(arr: &mut Vec<i32>, l: usize, r: usize) -> i32 {
        let mut result = 0;
        if l < r {
            let m = l + ((r - l) >> 1);

            let lower =  Self::merge_sort(arr, l, m);
            let high =  Self::merge_sort(arr, m + 1, r);

            result = lower + high + Self::merge(arr, l, m, r);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_inversions_1() {
        assert_eq!(Solution::count_inversions(vec![1, 1, 1, 2, 2]), 0);
    }

    #[test]
    fn count_inversions_2() {
        assert_eq!(Solution::count_inversions(vec![2, 1, 3, 1, 2]), 4);
    }
}