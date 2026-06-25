# 543. Diameter of Binary Tree
# Easy | Tree | Depth-First Search | Binary Tree
# https://leetcode.com/problems/diameter-of-binary-tree/
#
# Given the `root` of a binary tree, return *the length of the **diameter** of
# the tree*.
#
# The **diameter** of a binary tree is the **length** of the longest path
# between any two nodes in a tree. This path may or may not pass through the
# `root`.
#
# The **length** of a path between two nodes is represented by the number of
# edges between them.
#
# **Example 1:**
#
# ![](https://assets.leetcode.com/uploads/2021/03/06/diamtree.jpg)
#
# ```
# Input: root = [1,2,3,4,5]
# Output: 3
# Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
#
# ```
#
# **Example 2:**
#
# ```
# Input: root = [1,2]
# Output: 1
#
# ```
#
# **Constraints:**
#
# * The number of nodes in the tree is in the range `[1, 10<sup>4</sup>]`.
# * `-100 <= Node.val <= 100`

from typing import Dict, List, Optional

import pytest

from lc_helpers import TreeNode, to_level, tree_of

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


def dfs(node: TreeNode | None) -> tuple[int, int]:
    if node:
        (lh, ld) = dfs(node.left)
        (rh, rd) = dfs(node.right)
        return (1 + max(lh, rh), max(ld, rd, lh + rh))
    else:
        return (0, 0)


class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        return dfs(root)[1]


@pytest.mark.parametrize(
    "root, expected",
    [
        ([1, 2, 3, 4, 5], 3),
        ([1, 2], 1),
    ],
)
def test_cases(root, expected):
    assert Solution().diameterOfBinaryTree(tree_of(root)) == expected
