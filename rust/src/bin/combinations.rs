//! 77. Combinations
//! Medium | Backtracking
//! https://leetcode.com/problems/combinations/
//!
//! Given two integers `n` and `k`, return *all possible combinations of* `k`
//! *numbers chosen from the range* `[1, n]`.
//!
//! You may return the answer in **any order**.
//!
//! **Example 1:**
//!
//! ```
//! Input: n = 4, k = 2
//! Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
//! Explanation: There are 4 choose 2 = 6 total combinations.
//! Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: n = 1, k = 1
//! Output: [[1]]
//! Explanation: There is 1 choose 1 = 1 total combination.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= n <= 20`
//! * `1 <= k <= n`

struct Solution;

// [1, 2, 3, 4]
// [1] -> [2, 3, 4] => [1, 2] -> [3, 4] | [1, 3] -> [4] | [1, 4] -> [] => X
// [2] -> [3, 4] => [2, 3] -> [4] | [2, 4] -> [] => X
// [3] -> [4] => [3, 4] -> [] => X
// [4] => X
fn backtrack(start: i32, n: i32, k: usize, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if k == cur.len() {
        res.push(cur.clone());
        return;
    }

    for num in start..=n - (k - cur.len()) as i32 + 1 {
        cur.push(num);
        backtrack(num + 1, n, k, cur, res);
        cur.pop();
    }
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        backtrack(1, n, k as usize, &mut vec![], &mut res);
        res
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(4, 2, vec![vec![1,2],vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![3,4]])]
    #[case(1, 1, vec![vec![1]])]
    fn cases(#[case] n: i32, #[case] k: i32, #[case] expected: Vec<Vec<i32>>) {
        let canon = |mut v: Vec<Vec<i32>>| {
            v.iter_mut().for_each(|c| c.sort());
            v.sort();
            v
        };
        assert_eq!(canon(expected), canon(Solution::combine(n, k)));
    }
}
