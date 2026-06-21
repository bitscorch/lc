//! 235. Lowest Common Ancestor of a Binary Search Tree
//! Medium | Tree | Depth-First Search | Binary Search Tree | Binary Tree
//! https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
//!
//! Given a binary search tree (BST), find the lowest common ancestor (LCA) node
//! of two given nodes in the BST.
//!
//! According to the [definition of LCA on
//! Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor): “The
//! lowest common ancestor is defined between two nodes `p` and `q` as the
//! lowest node in `T` that has both `p` and `q` as descendants (where we allow
//! **a node to be a descendant of itself**).”
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2018/12/14/binarysearchtree_improved.png)
//!
//! ```
//! Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
//! Output: 6
//! Explanation: The LCA of nodes 2 and 8 is 6.
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2018/12/14/binarysearchtree_improved.png)
//!
//! ```
//! Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
//! Output: 2
//! Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself according to the LCA definition.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: root = [2,1], p = 2, q = 1
//! Output: 2
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[2, 10<sup>5</sup>]`.
//! * `-10<sup>9</sup> <= Node.val <= 10<sup>9</sup>`
//! * All `Node.val` are **unique**.
//! * `p != q`
//! * `p` and `q` will exist in the BST.

use lc::TreeNode;

struct Solution;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.as_ref().unwrap().borrow().val;
        let q = q.as_ref().unwrap().borrow().val;
        let mut cur = root;

        while let Some(node) = cur {
            let n = node.borrow();
            cur = if p < n.val && q < n.val {
                n.left.clone()
            } else if p > n.val && q > n.val {
                n.right.clone()
            } else {
                drop(n);
                return Some(node);
            };
        }
        None
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::vec_to_tree;
    use rstest::rstest;

    #[rstest]
    #[case(vec![Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)], vec![Some(2)], vec![Some(8)], 6)]
    #[case(vec![Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)], vec![Some(2)], vec![Some(4)], 2)]
    #[case(vec![Some(2), Some(1)], vec![Some(2)], vec![Some(1)], 2)]
    fn cases(
        #[case] root: Vec<Option<i32>>,
        #[case] p: Vec<Option<i32>>,
        #[case] q: Vec<Option<i32>>,
        #[case] expected: i32,
    ) {
        let lca =
            Solution::lowest_common_ancestor(vec_to_tree(root), vec_to_tree(p), vec_to_tree(q));
        assert_eq!(expected, lca.unwrap().borrow().val);
    }
}
