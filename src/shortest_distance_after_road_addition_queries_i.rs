// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-i/description/

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
       let mut adj_list = vec![Vec::new(); n as usize];
        (0..n - 1).for_each(|node| adj_list[node as usize].push(node + 1));
        let mut results = Vec::new();

        queries.iter().for_each(|q| {
            adj_list[q[0] as usize].push(q[1]);
            results.push(dijkstra(&adj_list))
        });

        results
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    cost: i32,
    position: i32
}

impl PartialOrd<Self> for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn dijkstra(adj_list: &Vec<Vec<i32>>) -> i32 {
    let mut dist = (0..adj_list.len()).map(|_| i32::MAX).collect::<Vec<i32>>();
    dist[0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Edge { position: 0, cost: 0});

    while let Some(Edge { position, cost }) = heap.pop() {
        if cost > dist[position as usize] { continue; }

        for cur_pos in &adj_list[position as usize] {
            let next = Edge { position: *cur_pos, cost: 1 + cost };

            if next.cost < dist[next.position as usize] {
                dist[next.position as usize] = next.cost;
                heap.push(next);
            }
        }
    }

    dist[adj_list.len() - 1]
}

#[cfg(test)]
mod tests {
    use crate::shortest_distance_after_road_addition_queries_i::Solution;

    #[test]
    fn test_one() {
        let result = Solution::shortest_distance_after_queries(5, vec![vec![2,4], vec![0,2], vec![0,4]]);
        assert_eq!(vec![3, 2, 1], result);
    }
}