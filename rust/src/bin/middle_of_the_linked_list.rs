//! 876. Middle of the Linked List
//! Easy | Linked List | Two Pointers
//! https://leetcode.com/problems/middle-of-the-linked-list/
//!
//! Given the `head` of a singly linked list, return *the middle node of the
//! linked list*.
//!
//! If there are two middle nodes, return **the second middle** node.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/07/23/lc-midlist1.jpg)
//!
//! ```
//! Input: head = [1,2,3,4,5]
//! Output: [3,4,5]
//! Explanation: The middle node of the list is node 3.
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/07/23/lc-midlist2.jpg)
//!
//! ```
//! Input: head = [1,2,3,4,5,6]
//! Output: [4,5,6]
//! Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the list is in the range `[1, 100]`.
//! * `1 <= Node.val <= 100`

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
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n = -1;
        let mut curr = head.clone();

        while let Some(node) = curr {
            curr = node.next;
            n += 1;
        }

        curr = head;

        for _ in 0..(n / 2 + n % 2) {
            curr = curr.unwrap().next;
        }

        curr
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::{list_to_vec, vec_to_list};
    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4,5], vec![3,4,5])]
    #[case(vec![1,2,3,4,5,6], vec![4,5,6])]
    fn cases(#[case] head: Vec<i32>, #[case] expected: Vec<i32>) {
        assert_eq!(
            expected,
            list_to_vec(Solution::middle_node(vec_to_list(head)))
        );
    }
}
