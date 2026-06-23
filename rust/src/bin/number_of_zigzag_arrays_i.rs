//! 3699. Number of ZigZag Arrays I
//!
//! Hard | Dynamic Programming | Prefix Sum
//! https://leetcode.com/problems/number-of-zigzag-arrays-i/
//!
//! You are given three integers `n`, `l`, and `r`.
//!
//! A **ZigZag** array of length `n` is defined as follows:
//!
//! * Each element lies in the range `[l, r]`.
//! * No **two** adjacent elements are equal.
//! * No **three** consecutive elements form a **strictly increasing** or **strictly decreasing** sequence.
//!
//! Return the total number of valid **ZigZag** arrays.
//!
//! Since the answer may be large, return it **modulo** `10<sup>9</sup> + 7`.
//!
//! A **sequence** is said to be **strictly increasing** if each element is
//! strictly greater than its previous one (if exists).
//!
//! A **sequence** is said to be **strictly decreasing** if each element is
//! strictly smaller than its previous one (if exists).
//!
//! **Example 1:**
//!
//! **Input:** n = 3, l = 4, r = 5
//!
//! **Output:** 2
//!
//! **Explanation:**
//!
//! There are only 2 valid ZigZag arrays of length `n = 3` using values in the
//! range `[4, 5]`:
//!
//! * `[4, 5, 4]`
//! * `[5, 4, 5]`​​​​​​​
//!
//! **Example 2:**
//!
//! **Input:** n = 3, l = 1, r = 3
//!
//! **Output:** 10
//!
//! **Explanation:**
//!
//! There are 10 valid ZigZag arrays of length `n = 3` using values in the range
//! `[1, 3]`:
//!
//! * `[1, 2, 1]`, `[1, 3, 1]`, `[1, 3, 2]`
//! * `[2, 1, 2]`, `[2, 1, 3]`, `[2, 3, 1]`, `[2, 3, 2]`
//! * `[3, 1, 2]`, `[3, 1, 3]`, `[3, 2, 3]`
//!
//! All arrays meet the ZigZag conditions.
//!
//! **Constraints:**
//!
//! * `3 <= n <= 2000`
//! * `1 <= l < r <= 2000`

struct Solution;

// state - remember the up/down of the last element
// 1 (u / d) | 2 (u / d) | 3 (u / d)
// 1 u -> X
// 2 u -> 1 d
// 3 u -> 1 d | 2 d
// 1 d -> 2 u | 3 u
// 2 d -> 3 u
// 3 d -> X
//
// 1 u -> 3 - 1 => 0..2 =>
impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const M: i64 = 1_000_000_007;
        let cnt = (r - l + 1) as usize;
        // down[j] == up[cnt-1-j]
        // ans = sum(up) * 2
        let mut up = vec![1i64; cnt];
        let mut next = vec![0i64; cnt];

        // O(n x m)
        for _ in 1..n {
            let mut acc = 0i64;
            for (slot, &v) in next.iter_mut().zip(up.iter().rev()) {
                *slot = acc;
                acc = (acc + v) % M;
            }
            std::mem::swap(&mut up, &mut next);
        }

        (2 * (up.iter().sum::<i64>() % M) % M) as i32
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(3, 4, 5, 2)]
    #[case(3, 1, 3, 10)]
    fn cases(#[case] n: i32, #[case] l: i32, #[case] r: i32, #[case] expected: i32) {
        assert_eq!(expected, Solution::zig_zag_arrays(n, l, r));
    }

    #[test]
    fn perf_max_bounds() {
        use std::time::Instant;
        let start = Instant::now();
        let result = Solution::zig_zag_arrays(2000, 1, 2000);
        let elapsed = start.elapsed();
        println!("max bounds: {elapsed:?}, result = {result}");
        assert!(
            elapsed.as_millis() < 500,
            "too slow ({elapsed:?}) - not O(n*m)"
        );
    }
}
