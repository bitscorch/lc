//! 981. Time Based Key-Value Store
//! Medium | Hash Table | String | Binary Search | Design
//! https://leetcode.com/problems/time-based-key-value-store/
//!
//! Design a time-based key-value data structure that can store multiple values
//! for the same key at different time stamps and retrieve the key's value at a
//! certain timestamp.
//!
//! Implement the `TimeMap` class:
//!
//! * `TimeMap()` Initializes the object of the data structure.
//! * `void set(String key, String value, int timestamp)` Stores the key `key` with the value `value` at the given time `timestamp`.
//! * `String get(String key, int timestamp)` Returns a value such that `set` was called previously, with `timestamp_prev <= timestamp`. If there are multiple such values, it returns the value associated with the largest `timestamp_prev`. If there are no values, it returns `""`.
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["TimeMap", "set", "get", "get", "set", "get", "get"]
//! [[], ["foo", "bar", 1], ["foo", 1], ["foo", 3], ["foo", "bar2", 4], ["foo", 4], ["foo", 5]]
//! Output
//! [null, null, "bar", "bar", null, "bar2", "bar2"]
//!
//! Explanation
//! TimeMap timeMap = new TimeMap();
//! timeMap.set("foo", "bar", 1);  // store the key "foo" and value "bar" along with timestamp = 1.
//! timeMap.get("foo", 1);         // return "bar"
//! timeMap.get("foo", 3);         // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
//! timeMap.set("foo", "bar2", 4); // store the key "foo" and value "bar2" along with timestamp = 4.
//! timeMap.get("foo", 4);         // return "bar2"
//! timeMap.get("foo", 5);         // return "bar2"
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= key.length, value.length <= 100`
//! * `key` and `value` consist of lowercase English letters and digits.
//! * `1 <= timestamp <= 10<sup>7</sup>`
//! * All the timestamps `timestamp` of `set` are strictly increasing.
//! * At most `2 * 10<sup>5</sup>` calls will be made to `set` and `get`.

struct Solution;

use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>,
}

fn binary_search(timestamps: &[(String, i32)], timestamp: i32) -> String {
    let (mut lo, mut hi) = (0, timestamps.len());
    let mut ans = "";

    while lo < hi {
        let m = lo + (hi - lo) / 2;
        if timestamps[m].1 <= timestamp {
            ans = timestamps[m].0.as_str();
            lo = m + 1;
        } else {
            hi = m;
        }
    }
    ans.to_string()
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let timestamps = self.map.entry(key).or_insert(vec![]);
        timestamps.push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        // binary search
        if let Some(timestamps) = self.map.get(&key) {
            binary_search(timestamps, timestamp)
        } else {
            "".to_string()
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut tm = TimeMap::new();
        tm.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(tm.get("foo".to_string(), 1), "bar");
        assert_eq!(tm.get("foo".to_string(), 3), "bar"); // no ts 3 or 2 → falls back to ts 1
        tm.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(tm.get("foo".to_string(), 4), "bar2");
        assert_eq!(tm.get("foo".to_string(), 5), "bar2");
    }

    #[test]
    fn missing_key_and_query_before_first() {
        let mut tm = TimeMap::new();
        assert_eq!(tm.get("nope".to_string(), 1), ""); // key never set → ""
        tm.set("foo".to_string(), "bar".to_string(), 10);
        assert_eq!(tm.get("foo".to_string(), 5), ""); // query before earliest ts → ""
        assert_eq!(tm.get("foo".to_string(), 10), "bar"); // exact match → "bar"
        assert_eq!(tm.get("foo".to_string(), 100), "bar"); // after latest ts → "bar"
    }
}
