//! 2900. Longest Unequal Adjacent Groups Subsequence I
//! Easy | Array | String | Dynamic Programming | Greedy
//! https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i/
//!
//! You are given a string array `words` and a **binary** array `groups` both of
//! length `n`.
//!
//! A subsequence of `words` is **alternating** if for any two *consecutive*
//! strings in the sequence, their corresponding elements at the *same* indices
//! in `groups` are **different** (that is, there *cannot* be consecutive 0 or
//! 1).
//!
//! Your task is to select the **longest alternating** subsequence from `words`.
//!
//! Return *the selected subsequence. If there are multiple answers, return
//! **any** of them.*
//!
//! **Note:** The elements in `words` are distinct.
//!
//! **Example 1:**
//!
//! **Input:** words = ["e","a","b"], groups = [0,0,1]
//!
//! **Output:** ["e","b"]
//!
//! **Explanation:** A subsequence that can be selected is `["e","b"]` because `groups[0] != groups[2]`. Another subsequence that can be selected is `["a","b"]` because `groups[1] != groups[2]`. It can be demonstrated that the length of the longest subsequence of indices that satisfies the condition is `2`.
//!
//! **Example 2:**
//!
//! **Input:** words = ["a","b","c","d"], groups = [1,0,1,1]
//!
//! **Output:** ["a","b","c"]
//!
//! **Explanation:** A subsequence that can be selected is `["a","b","c"]` because `groups[0] != groups[1]` and `groups[1] != groups[2]`. Another subsequence that can be selected is `["a","b","d"]` because `groups[0] != groups[1]` and `groups[1] != groups[3]`. It can be shown that the length of the longest subsequence of indices that satisfies the condition is `3`.
//!
//! **Constraints:**
//!
//! * `1 <= n == words.length == groups.length <= 100`
//! * `1 <= words[i].length <= 10`
//! * `groups[i]` is either `0` or `1.`
//! * `words` consists of **distinct** strings.
//! * `words[i]` consists of lowercase English letters.

struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let (mut out, mut last) = (vec![words[0].clone()], groups[0]);
        for i in 1..groups.len() {
            if groups[i] != last {
                out.push(words[i].clone());
                last = groups[i];
            }
        }
        out
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["e", "a", "b"], vec![0, 0, 1], vec!["e", "b"])]
    #[case(vec!["a", "b", "c", "d"], vec![1, 0, 1, 1], vec!["a", "b", "c"])]
    #[case(vec!["a"], vec![0], vec!["a"])]
    #[case(vec!["x", "y"], vec![1, 1], vec!["x"])]
    fn cases(#[case] words: Vec<&str>, #[case] groups: Vec<i32>, #[case] expected: Vec<&str>) {
        let words = words.into_iter().map(str::to_string).collect();
        assert_eq!(Solution::get_longest_subsequence(words, groups), expected);
    }
}
