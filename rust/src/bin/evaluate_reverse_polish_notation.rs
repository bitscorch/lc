//! 150. Evaluate Reverse Polish Notation
//! Medium | Array | Math | Stack
//! https://leetcode.com/problems/evaluate-reverse-polish-notation/
//!
//! You are given an array of strings `tokens` that represents an arithmetic
//! expression in a [Reverse Polish
//! Notation](http://en.wikipedia.org/wiki/Reverse_Polish_notation).
//!
//! Evaluate the expression. Return *an integer that represents the value of the
//! expression*.
//!
//! **Note** that:
//!
//! * The valid operators are `'+'`, `'-'`, `'*'`, and `'/'`.
//! * Each operand may be an integer or another expression.
//! * The division between two integers always **truncates toward zero**.
//! * There will not be any division by zero.
//! * The input represents a valid arithmetic expression in a reverse polish notation.
//! * The answer and all the intermediate calculations can be represented in a **32-bit** integer.
//!
//! **Example 1:**
//!
//! ```
//! Input: tokens = ["2","1","+","3","*"]
//! Output: 9
//! Explanation: ((2 + 1) * 3) = 9
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: tokens = ["4","13","5","/","+"]
//! Output: 6
//! Explanation: (4 + (13 / 5)) = 6
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
//! Output: 22
//! Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
//! = ((10 * (6 / (12 * -11))) + 17) + 5
//! = ((10 * (6 / -132)) + 17) + 5
//! = ((10 * 0) + 17) + 5
//! = (0 + 17) + 5
//! = 17 + 5
//! = 22
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= tokens.length <= 10<sup>4</sup>`
//! * `tokens[i]` is either an operator: `"+"`, `"-"`, `"*"`, or `"/"`, or an integer in the range `[-200, 200]`.

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            if let Ok(num) = token.parse::<i32>() {
                stack.push(num)
            } else {
                let (x, y) = (stack.pop().unwrap(), stack.pop().unwrap());
                let res = match token.as_str() {
                    "+" => y + x,
                    "-" => y - x,
                    "*" => y * x,
                    "/" => y / x,
                    _ => unreachable!(),
                };
                stack.push(res);
            }
        }

        stack.pop().unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["2".to_string(),"1".to_string(),"+".to_string(),"3".to_string(),"*".to_string()], 9)]
    #[case(vec!["4".to_string(),"13".to_string(),"5".to_string(),"/".to_string(),"+".to_string()], 6)]
    #[case(vec!["10".to_string(),"6".to_string(),"9".to_string(),"3".to_string(),"+".to_string(),"-11".to_string(),"*".to_string(),"/".to_string(),"*".to_string(),"17".to_string(),"+".to_string(),"5".to_string(),"+".to_string()], 22)]
    fn cases(#[case] tokens: Vec<String>, #[case] expected: i32) {
        assert_eq!(expected, Solution::eval_rpn(tokens));
    }
}
