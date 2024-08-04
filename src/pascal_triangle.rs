// https://leetcode.com/problems/pascals-triangle/description/?envType=company&envId=apple&favoriteSlug=apple-thirty-days

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = Vec::new();

        for row in 0..num_rows as usize {
            let row_size = row + 1;
            let mut vec = vec![0; row_size];

            for col in 0..row_size {
                vec[col] = if row == 0 || col == 0 || result[row - 1].len() - 1 < col {
                     1
                } else {
                    result[row-1][col - 1] + result[row-1][col]
                }
            }

            result.push(vec);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::pascal_triangle::Solution;

    #[test]
    fn test_one() {
        assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]], Solution::generate(5));
    }
}