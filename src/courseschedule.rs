use std::collections::HashMap;

struct Solution;

// need to review topological sort via dfs

impl Solution {
    // O(num_courses^2 + prerequisites.len())
    // O(num_courses^2)
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut visited: Vec<bool> = vec![false; num_courses as usize];
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut checked = vec![false; num_courses as usize];

        for i in prerequisites {
            if !adj.contains_key(&i[1]) {
                adj.insert(i[1], Vec::new());
            }

            (*adj.get_mut(&i[1]).unwrap()).push(i[0]);
        }

        for i in 0..num_courses {
            if !visited[i as usize] {
                if Self::is_cyclic(i as usize, &mut adj, &mut visited) {
                    return false;
                }
            }
        }

        true
    }

    // DFS <- Visitado
    //
    //           H
    //          /
    //      B -- C
    //   < /       \
    // A <         F - G
    //    \
    //     D -- E
    //
    //     D
    //   /
    //  A - B - C
    //  \      /
    //   \____/

    fn is_cyclic(course: usize, adj: &HashMap<i32, Vec<i32>>, visited: &mut Vec<bool>) -> bool {
        if visited[course] {
            return true;
        }

        visited[course] = true;

        let mut ret = false;
        if adj.contains_key(&(course as i32)) {
            for i in adj.get(&(course as i32)).unwrap() {
                if Self::is_cyclic(*i as usize, adj, visited) {
                    ret = true;
                    break;
                }
            }
        }

        visited[course] = false;

        ret
    }

    // O(course + adj)
    fn is_cyclic_pos_order(
        course: usize,
        adj: &HashMap<i32, Vec<i32>>,
        visited: &mut Vec<bool>,
        path: &mut Vec<bool>,
    ) -> bool {
        if visited[course] {
            return false;
        }

        if path[course] {
            return true;
        }

        path[course] = true;

        let mut ret = false;
        if adj.contains_key(&(course as i32)) {
            for i in adj.get(&(course as i32)).unwrap() {
                if Self::is_cyclic_pos_order(*i as usize, adj, visited, path) {
                    ret = true;
                    break;
                }
            }
        }

        path[course] = false;
        visited[course] = true;

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_finish_1() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    }

    #[test]
    fn can_finish_2() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    }

    #[test]
    fn can_finish_3() {
        assert_eq!(Solution::can_finish(2, vec![vec![0, 1]]), true);
    }
}
