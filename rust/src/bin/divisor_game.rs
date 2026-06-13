//! 1025. Divisor Game
//! Easy | Math | Dynamic Programming | Brainteaser | Game Theory
//! https://leetcode.com/problems/divisor-game/
//!
//! Alice and Bob take turns playing a game, with Alice starting first.
//!
//! Initially, there is a number `n` on the chalkboard. On each player's turn,
//! that player makes a move consisting of:
//!
//! * Choosing any integer `x` with `0 < x < n` and `n % x == 0`.
//! * Replacing the number `n` on the chalkboard with `n - x`.
//!
//! Also, if a player cannot make a move, they lose the game.
//!
//! Return `true` *if and only if Alice wins the game, assuming both players
//! play optimally*.
//!
//! **Example 1:**
//!
//! ```
//! Input: n = 2
//! Output: true
//! Explanation: Alice chooses 1, and Bob has no more moves.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: n = 3
//! Output: false
//! Explanation: Alice chooses 1, Bob chooses 1, and Alice has no more moves.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= n <= 1000`

struct Solution;

// 1 => false
// 2 => A 1 => true
// 3 => A 1 B 1 => false
// 4 => A 2 B 1 A 1=> true
// 5 => A 1 B 1 A 1 B 1 A 1 => false
//
// f(1) = false
// f(2) = !f(1) = true
// f(3) = !f(2) = false
impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, true)]
    #[case(3, false)]
    fn cases(#[case] n: i32, #[case] expected: bool) {
        assert_eq!(expected, Solution::divisor_game(n));
    }
}
