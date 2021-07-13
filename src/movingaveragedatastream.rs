use std::collections::VecDeque;

struct MovingAverage {
    content: VecDeque<i32>,
    sum: i32,
    max_items: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        MovingAverage {
            content: VecDeque::new(),
            sum: 0,
            max_items: size as usize,
        }
    }

    fn add_to_list(&mut self, val: i32) {
        self.sum += val;
        self.content.push_front(val);

        if self.content.len() > self.max_items {
            let val_to_remove = self.content.pop_back().unwrap();
            self.sum -= val_to_remove;
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.add_to_list(val);
        self.sum as f64 / self.content.len() as f64
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */

#[test]
fn test_one() {
    let mut ma = MovingAverage::new(3);

    assert_eq!(1.0, ma.next(1));
    assert_eq!(5.5, ma.next(10));
    assert_eq!(4.666666666666667, ma.next(3));
    assert_eq!(6.0, ma.next(5));
}
