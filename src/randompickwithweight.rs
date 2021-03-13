use std::collections::BTreeMap;

struct Solution {
    sum: u32,
    weight: BTreeMap<u32, i32>,
}

extern "C" {
    fn rand() -> u32;
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut sum = 0;
        let mut weight = BTreeMap::new();

        for (idx, &val) in w.iter().enumerate() {
            sum += val as u32;

            weight.insert(sum, idx as i32);
        }

        Solution { sum, weight }
    }

    fn get_random_number(&self) -> u32 {
        let r = unsafe { rand() };
        r % self.sum + 1
    }

    fn pick_index(&self) -> i32 {
        *self
            .weight
            .range(self.get_random_number()..)
            .next()
            .unwrap()
            .1
    }
}
