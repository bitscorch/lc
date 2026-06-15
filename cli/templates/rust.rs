//! {{number}}. {{title}}
//! {{difficulty}} | {{tags}}
//! https://leetcode.com/problems/{{slug}}/
//!
{{description}}

{{imports}}struct Solution;

{{snippet}}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
{{test_imports}}    use rstest::rstest;

{{tests}}
}
