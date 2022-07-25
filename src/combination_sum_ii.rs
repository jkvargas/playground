struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            fn backtrack(sub: &[i32], candidates: &[i32], target: i32, res: &mut Vec<Vec<i32>>) {
                let sum = sub.iter().sum::<i32>();
                if sum == target {
                    res.push(sub.to_vec());
                    return;
                } else if sum > target {
                    return;
                }

                for (i, v) in candidates.iter().enumerate() {
                    if i > 0 && candidates[i - 1] == *v {
                        continue;
                    }
                    let mut s = sub.to_vec();
                    s.push(*v);
                    backtrack(&s, &candidates[i + 1..], target, res);
                }
            }
            let mut candidates = candidates;
            candidates.sort();
            let mut res: Vec<Vec<i32>> = vec![];
            backtrack(&vec![], &candidates, target, &mut res);
            res
    }
}

#[cfg(test)]
mod tests {
    use crate::combination_sum_ii::Solution;

    #[test]
    fn test_one() {
        assert_eq!(vec![
            vec![1,1,6],
            vec![1,2,5],
            vec![1,7],
            vec![2,6]
        ], Solution::combination_sum2(vec![10,1,2,7,6,1,5], 8));
    }
}

// private void backtrack(LinkedList<Integer> comb,
// int remain, int curr,
// List<int[]> counter,
// List<List<Integer>> results) {
//
// if (remain <= 0) {
// if (remain == 0) {
// // make a deep copy of the current combination.
// results.add(new ArrayList<Integer>(comb));
// }
// return;
// }
//
// for (int nextCurr = curr; nextCurr < counter.size(); ++nextCurr) {
// int[] entry = counter.get(nextCurr);
// Integer candidate = entry[0], freq = entry[1];
//
// if (freq <= 0)
// continue;
//
// // add a new element to the current combination
// comb.addLast(candidate);
// counter.set(nextCurr, new int[]{candidate, freq - 1});
//
// // continue the exploration with the updated combination
// backtrack(comb, remain - candidate, nextCurr, counter, results);
//
// // backtrack the changes, so that we can try another candidate
// counter.set(nextCurr, new int[]{candidate, freq});
// comb.removeLast();
// }
// }
// }