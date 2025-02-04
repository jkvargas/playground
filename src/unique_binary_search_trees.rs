// https://leetcode.com/problems/unique-binary-search-trees/

struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut G = vec![0; n as usize + 1];
        G[0] = 1;
        G[1] = 1;

        for node in 2..=n {
            for root in 1..=node {
                G[node as usize] += G[root as usize - 1] * G[node as usize - root as usize];
            }
        }

        return G[n as usize];
    }
}
