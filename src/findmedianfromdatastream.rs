struct MedianFinder {
    items: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder { items: Vec::new() }
    }

    fn add_num(&mut self, num: i32) {
        if (self.items.is_empty()) {
            self.items.push(num);
        } else {
            match self.items.binary_search(&num) {
                Ok(pos) => {
                    self.items.insert(pos, num);
                }
                Err(pos) => {
                    self.items.insert(pos, num);
                }
            }
        }
    }

    fn find_median(&self) -> f64 {
        let len = self.items.len();

        if len & 1 != 0 {
            *self.items.get(len / 2).unwrap() as f64
        } else {
            (*self.items.get(len / 2 - 1).unwrap() as f64
                + *self.items.get(len / 2).unwrap() as f64)
                * 0.5
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
#[test]
pub fn how_act() {
    let mut med = MedianFinder::new();

    med.add_num(1);
    med.add_num(2);
    assert_eq!(1.5, med.find_median());
    med.add_num(3);
    assert_eq!(2.0, med.find_median());
}
