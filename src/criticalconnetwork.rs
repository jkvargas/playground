use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut l = vec![Vec::new(); n as usize];
        let mut res = Vec::new();
        let mut visited = vec![-1; n as usize];
        let mut low = vec![0; n as usize];

        for c in connections {
            let from = c[0] as usize;
            let to = c[1] as usize;

            l[from].push(to);
            l[to].push(from);
        }

        for i in 0..n as usize {
            if visited[i] == -1 {
                dfs(&mut res, &mut visited, &mut low, &l, i, i, 0);
            }
        }

        res
    }
}

fn dfs(
    res: &mut Vec<Vec<i32>>,
    visited: &mut Vec<i32>,
    low: &mut Vec<i32>,
    l: &Vec<Vec<usize>>,
    u: usize,
    prev: usize,
    time: i32,
) {
    visited[u] = time;
    low[u] = time;

    for &v in &l[u] {
        if v == prev {
            continue;
        }

        if visited[v] != -1 {
            low[u] = std::cmp::min(low[u], visited[v]);
            continue;
        }

        dfs(res, visited, low, l, v, u, time + 1);
        low[u] = std::cmp::min(low[u], low[v]);

        if low[v] > visited[u] {
            res.push(vec![u as i32, v as i32]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn critical_connections_1() {
        assert_eq!(Solution::critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]), vec![vec![1, 3]]);
    }

    #[test]
    fn critical_connections_2() {
        assert_eq!(Solution::critical_connections(6, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3], vec![3, 4], vec![4, 5], vec![5, 3]]), vec![vec![1, 3]]);
    }
}