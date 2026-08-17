# 236. Lowest Common Ancestor of a Binary Tree
# Medium | Tree | Depth-First Search | Binary Tree | Binary Lifting | Lowest Common Ancestor
# https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
#
# Given a binary tree, find the lowest common ancestor (LCA) of two given
# nodes in the tree.
#
# According to the [definition of LCA on
# Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor): “The
# lowest common ancestor is defined between two nodes `p` and `q` as the
# lowest node in `T` that has both `p` and `q` as descendants (where we allow
# **a node to be a descendant of itself**).”
#
# **Example 1:**
#
# ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)
#
# ```
# Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
# Output: 3
# Explanation: The LCA of nodes 5 and 1 is 3.
#
# ```
#
# **Example 2:**
#
# ![](https://assets.leetcode.com/uploads/2018/12/14/binarytree.png)
#
# ```
# Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
# Output: 5
# Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
#
# ```
#
# **Example 3:**
#
# ```
# Input: root = [1,2], p = 1, q = 2
# Output: 1
#
# ```
#
# **Constraints:**
#
# * The number of nodes in the tree is in the range `[2, 10<sup>5</sup>]`.
# * `-10<sup>9</sup> <= Node.val <= 10<sup>9</sup>`
# * All `Node.val` are **unique**.
# * `p != q`
# * `p` and `q` will exist in the tree.

from typing import Dict, List, Optional

import pytest

from lc_helpers import TreeNode, tree_of

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def lowestCommonAncestor(
        self, root: "TreeNode", p: "TreeNode", q: "TreeNode"
    ) -> "TreeNode":
        def dfs(node: TreeNode | None) -> TreeNode | None:
            if node is None:
                return None

            if node.val == p.val or node.val == q.val:
                return node

            left, right = dfs(node.left), dfs(node.right)
            if left is not None and right is not None:
                return node
            elif left is not None:
                return left
            elif right is not None:
                return right

        ans = dfs(root)
        return ans


@pytest.mark.parametrize(
    "root, p, q, expected",
    [
        ([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], 5, 1, 3),
        ([3, 5, 1, 6, 2, 0, 8, None, None, 7, 4], 5, 4, 5),
        ([1, 2], 1, 2, 1),
    ],
)
def test_cases(root, p, q, expected):
    tree = tree_of(root)
    assert (
        Solution().lowestCommonAncestor(tree, TreeNode(p), TreeNode(q)).val == expected
    )
