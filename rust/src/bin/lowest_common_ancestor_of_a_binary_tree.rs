//! 236. Lowest Common Ancestor of a Binary Tree
//! Medium | Tree | Depth-First Search | Binary Tree | Binary Lifting | Lowest Common Ancestor
//! https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
//!
//! Given a binary tree, find the lowest common ancestor (LCA) of two given
//! nodes in the tree.
//!
//! According to the [definition of LCA on
//! Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor): “The
//! lowest common ancestor is defined between two nodes `p` and `q` as the
//! lowest node in `T` that has both `p` and `q` as descendants (where we allow
//! **a node to be a descendant of itself**).”
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)
//!
//! ```
//! Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
//! Output: 3
//! Explanation: The LCA of nodes 5 and 1 is 3.
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)
//!
//! ```
//! Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
//! Output: 5
//! Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: root = [1,2], p = 1, q = 2
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the tree is in the range `[2, 10<sup>5</sup>]`.
//! * `-10<sup>9</sup> <= Node.val <= 10<sup>9</sup>`
//! * All `Node.val` are **unique**.
//! * `p != q`
//! * `p` and `q` will exist in the tree.

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

type Node = Option<Rc<RefCell<TreeNode>>>;

fn dfs(node: Node, pv: i32, qv: i32) -> Node {
    let n = node.as_ref()?;
    let val = n.borrow().val;
    if val == pv || val == qv {
        return node;
    };

    let (left, right) = {
        let tmp = n.borrow();
        (
            dfs(tmp.left.clone(), pv, qv),
            dfs(tmp.right.clone(), pv, qv),
        )
    };

    match (left, right) {
        (Some(_l), Some(_r)) => node,
        (l, None) => l,
        (None, r) => r,
    }
}

impl Solution {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        dfs(
            root,
            p.unwrap().as_ref().borrow().val,
            q.unwrap().as_ref().borrow().val,
        )
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use lc::vec_to_tree;
    use rstest::rstest;

    #[rstest]
    #[case(vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)], vec![Some(5)], vec![Some(1)], 3)]
    #[case(vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)], vec![Some(5)], vec![Some(4)], 5)]
    #[case(vec![Some(1), Some(2)], vec![Some(1)], vec![Some(2)], 1)]
    fn cases(
        #[case] root: Vec<Option<i32>>,
        #[case] p: Vec<Option<i32>>,
        #[case] q: Vec<Option<i32>>,
        #[case] expected: i32,
    ) {
        let lca = Solution::lowest_common_ancestor(
            vec_to_tree(root),
            vec_to_tree(p),
            vec_to_tree(q),
        );
        assert_eq!(expected, lca.unwrap().borrow().val);
    }
}
