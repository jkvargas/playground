use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

/// marco // joga tenis // 1
///
/// marco 1 -- 1
/// Ok(pos) -> Err(pos) 0 2 3 4

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if !self.map.contains_key(&key) {
            self.map.insert(key.clone(), Vec::new());
        }

        self.map.get_mut(&key).unwrap().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if self.map.contains_key(&key) {
            return match self
                .map
                .get(&key)
                .unwrap()
                .binary_search_by(|x| (*x).0.cmp(&timestamp))
            {
                Ok(found) => self.map.get(&key).unwrap()[found].1.clone(),
                Err(not_found) => {
                    let reference = self.map.get(&key).unwrap();
                    if not_found > 0 {
                        return reference[not_found - 1].1.clone();
                    } else {
                        return "".to_string();
                    }
                }
            };
        }

        "".to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_1() {
        let mut timemap = TimeMap::new();

        timemap.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(timemap.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(timemap.get("foo".to_string(), 3), "bar".to_string());
        timemap.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(timemap.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(timemap.get("foo".to_string(), 5), "bar2".to_string());
    }

    #[test]
    fn timestamp_2() {
        let mut timemap = TimeMap::new();

        timemap.set("love".to_string(), "high".to_string(), 10);
        timemap.set("love".to_string(), "low".to_string(), 20);
        assert_eq!(timemap.get("love".to_string(), 5), "".to_string());
        assert_eq!(timemap.get("love".to_string(), 10), "high".to_string());
        assert_eq!(timemap.get("love".to_string(), 15), "high".to_string());
        assert_eq!(timemap.get("love".to_string(), 20), "low".to_string());
        assert_eq!(timemap.get("love".to_string(), 25), "low".to_string());
    }
}
