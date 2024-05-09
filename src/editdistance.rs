struct Solution;

impl Solution {
    pub fn min_distance_old(word1: String, word2: String) -> i32 {
        // Treat characters as raw bytes, as it allows us to directly access the underlying arrays:
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());

        // Allocate memory in one-go, as it is typically faster:
        let mut dist = Vec::with_capacity(word2.len() + 1);

        // Base case: we need to delete j characters in word2 in order to match the empty string word1:
        for j in 0..=word2.len() {
            dist.push(j)
        }

        // Use a second vector to store distances for i - 1.
        // This uses less memory than having a matrix of size (m, n),
        // and we always just use the previous row in the matrix anyway:
        let mut prev_dist = dist.clone();

        for i in 1..=word1.len() {
            for j in 0..=word2.len() {
                if j == 0 {
                    dist[j] += 1; // Base case: we need to insert a character in order to match word1.
                } else if word1[i - 1] == word2[j - 1] {
                    // No difference, don't increment the edit distance:
                    dist[j] = prev_dist[j - 1];
                } else {
                    // Either insert, delete or replace a character: increment the edit distance by one:
                    dist[j] = dist[j].min(dist[j - 1]).min(prev_dist[j - 1]) + 1;
                }
            }
            prev_dist.copy_from_slice(&dist); // Backup the distances for this row using memcpy.
        }
        dist[word2.len()] as i32
    }

    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1_vec : Vec<char> = word1.chars().collect();
        let word2_vec : Vec<char> = word2.chars().collect();
        let mut memo = vec![vec![-1; word2_vec.len() + 1]; word1_vec.len() + 1];
        Self::min_distance_recur(&word1_vec, &word2_vec, word1_vec.len(), word2_vec.len(), &mut memo)
    }

    fn min_distance_recur(word_one: &Vec<char>, word_two: &Vec<char>, word_one_index: usize, word_two_index: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if word_one_index == 0 {
            return word_two_index as i32;
        }

        if word_two_index == 0 {
            return word_one_index as i32;
        }

        if memo[word_one_index][word_two_index] != -1 {
            return memo[word_one_index][word_two_index];
        }

        let mut min_edit_distance = 0;
        if word_one[word_one_index - 1] == word_two[word_two_index - 1] {
            min_edit_distance = Self::min_distance_recur(word_one, word_two, word_one_index - 1, word_two_index - 1, memo);
        } else {
            let insert_op = Self::min_distance_recur(word_one, word_two, word_one_index, word_two_index - 1, memo);
            let delete_op = Self::min_distance_recur(word_one, word_two, word_one_index - 1, word_two_index, memo);
            let replace_op = Self::min_distance_recur(word_one, word_two, word_one_index - 1, word_two_index - 1, memo);
            min_edit_distance = insert_op.min(delete_op).min(replace_op) + 1;
        }

        memo[word_one_index][word_two_index] = min_edit_distance;
        return min_edit_distance;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn min_distance_1() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
    }

    #[test]
    pub fn min_distance_2() {
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
