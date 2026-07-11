# 98. Validate Binary Search Tree
# Medium | Tree | Depth-First Search | Binary Search Tree | Binary Tree
# https://leetcode.com/problems/validate-binary-search-tree/
#
# Given the `root` of a binary tree, *determine if it is a valid binary search
# tree (BST)*.
#
# A **valid BST** is defined as follows:
#
# * The left subtree of a node contains only nodes with keys **strictly less than** the node's key.
# * The right subtree of a node contains only nodes with keys **strictly greater than** the node's key.
# * Both the left and right subtrees must also be binary search trees.
#
# **Example 1:**
#
# ![](https://assets.leetcode.com/uploads/2020/12/01/tree1.jpg)
#
# ```
# Input: root = [2,1,3]
# Output: true
#
# ```
#
# **Example 2:**
#
# ![](https://assets.leetcode.com/uploads/2020/12/01/tree2.jpg)
#
# ```
# Input: root = [5,1,4,null,null,3,6]
# Output: false
# Explanation: The root node's value is 5 but its right child's value is 4.
#
# ```
#
# **Constraints:**
#
# * The number of nodes in the tree is in the range `[1, 10<sup>4</sup>]`.
# * `-2<sup>31</sup> <= Node.val <= 2<sup>31</sup> - 1`

from typing import Dict, List, Optional

import pytest

from lc_helpers import TreeNode, to_level, tree_of

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def check(node: TreeNode | None, lt: int | None, mt: int | None) -> bool:
            if node is None:
                return True

            if lt is not None and node.val >= lt:
                return False
            elif mt is not None and node.val <= mt:
                return False
            else:
                return check(node.left, node.val, mt) and check(
                    node.right, lt, node.val
                )

        return check(root, None, None)


@pytest.mark.parametrize(
    "root, expected",
    [
        ([2, 1, 3], True),
        ([5, 1, 4, None, None, 3, 6], False),
    ],
)
def test_cases(root, expected):
    assert Solution().isValidBST(tree_of(root)) == expected
