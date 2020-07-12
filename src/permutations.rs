use std::collections::HashMap;

struct Solution;

struct OrderedMap {
    map: HashMap<usize, usize>,
    vec: Vec<usize>
}

impl OrderedMap {
    pub fn new() -> OrderedMap {
        Self {
            map: HashMap::new(),
            vec: Vec::new()
        }
    }

    pub fn insert(&mut self, val: usize) {
        self.vec.push(val);
        self.map.insert(val, self.vec.len() - 1);
    }

    pub fn contains(&self, val: &usize) -> bool {
        self.map.contains_key(val)
    }

    pub fn remove(&mut self, val: &usize) {
        let last = self.vec.pop().unwrap();
        self.map.remove(&last);
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.vec.iter()
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results : Vec<Vec<i32>> = Vec::new();
        let mut ignore = OrderedMap::new();

        Self::find_permutations(&mut ignore, &nums, &mut results);

        results
    }

    fn find_permutations(ignore: &mut OrderedMap, original: &Vec<i32>, results: &mut Vec<Vec<i32>>) {
        if ignore.len() == original.len() {
            let mut res : Vec<i32> = ignore.iter().map(|x| original[*x]).collect();
            results.push(res);
            return;
        }

        for i in 0..original.len() {
            if !ignore.contains(&i) {
                ignore.insert(i);
                Self::find_permutations(ignore, original, results);
                ignore.remove(&i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute_1() {
        let result = Solution::permute(vec![1, 2, 3]);
        assert_eq!(result, vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]]);
    }

    #[test]
    fn permute_2() {
        let result = Solution::permute(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    }
}
