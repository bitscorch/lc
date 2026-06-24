//! 206. Reverse Linked List
//! Easy | Linked List | Recursion
//! https://leetcode.com/problems/reverse-linked-list/
//!
//! Given the `head` of a singly linked list, reverse the list, and return *the
//! reversed list*.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg)
//!
//! ```
//! Input: head = [1,2,3,4,5]
//! Output: [5,4,3,2,1]
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg)
//!
//! ```
//! Input: head = [1,2]
//! Output: [2,1]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: head = []
//! Output: []
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the list is the range `[0, 5000]`.
//! * `-5000 <= Node.val <= 5000`
//!
//! **Follow up:** A linked list can be reversed either iteratively or
//! recursively. Could you implement both?

use lc::ListNode;

struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// a -> b -> c -> d -> e -> None
// None, a -> b -> ...
// a -> None, b -> c -> ...
// b -> a -> None, c
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut node) = curr.take() {
            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::{list_to_vec, vec_to_list};
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4,5], vec![5,4,3,2,1])]
    #[case(vec![1,2], vec![2,1])]
    #[case(vec![], vec![])]
    fn cases(#[case] head: Vec<i32>, #[case] expected: Vec<i32>) {
        assert_eq!(
            expected,
            list_to_vec(Solution::reverse_list(vec_to_list(head)))
        );
    }
}
