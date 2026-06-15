//! 21. Merge Two Sorted Lists
//! Easy | Linked List | Recursion
//! https://leetcode.com/problems/merge-two-sorted-lists/
//!
//! You are given the heads of two sorted linked lists `list1` and `list2`.
//!
//! Merge the two lists into one **sorted** list. The list should be made by
//! splicing together the nodes of the first two lists.
//!
//! Return *the head of the merged linked list*.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg)
//!
//! ```
//! Input: list1 = [1,2,4], list2 = [1,3,4]
//! Output: [1,1,2,3,4,4]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: list1 = [], list2 = []
//! Output: []
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: list1 = [], list2 = [0]
//! Output: [0]
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in both lists is in the range `[0, 50]`.
//! * `-100 <= Node.val <= 100`
//! * Both `list1` and `list2` are sorted in **non-decreasing** order.

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
//
//
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(mut a), Some(mut b)) => {
                if a.val <= b.val {
                    a.next = Self::merge_two_lists(a.next, Some(b));
                    Some(a)
                } else {
                    b.next = Self::merge_two_lists(Some(a), b.next);
                    Some(b)
                }
            }
            (el, None) | (None, el) => el,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::{list_to_vec, vec_to_list};
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,4], vec![1,3,4], vec![1,1,2,3,4,4])]
    #[case(vec![], vec![], vec![])]
    #[case(vec![], vec![0], vec![0])]
    fn cases(#[case] list1: Vec<i32>, #[case] list2: Vec<i32>, #[case] expected: Vec<i32>) {
        assert_eq!(
            expected,
            list_to_vec(Solution::merge_two_lists(
                vec_to_list(list1),
                vec_to_list(list2)
            ))
        );
    }
}
