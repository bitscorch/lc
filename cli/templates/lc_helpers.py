"""Local helpers for LeetCode solutions that use custom types (linked lists,
binary trees). LeetCode's judge defines ListNode/TreeNode itself, so `lc submit`
strips these imports — they exist only so solutions run and tests pass locally."""

from collections import deque


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def list_of(values):
    """Build a linked list from a list (head = first element)."""
    head = None
    for v in reversed(values):
        head = ListNode(v, head)
    return head


def to_list(node):
    """Collect a linked list back into a plain list."""
    out = []
    while node:
        out.append(node.val)
        node = node.next
    return out


def cyclic_list(values, pos):
    """Build a linked list, then link the tail back to the node at index `pos`
    (`pos < 0` = no cycle). For cycle-detection problems, where the example's
    `pos` describes the cycle but is NOT a function argument — auto-generated
    tests can't handle this, so write those by hand using this helper."""
    head = list_of(values)
    if pos < 0 or head is None:
        return head
    tail, nodes = head, [head]
    while tail.next:
        tail = tail.next
        nodes.append(tail)
    tail.next = nodes[pos]
    return head


def tree_of(values):
    """Build a tree from LeetCode's level-order list (None = a missing node)."""
    if not values or values[0] is None:
        return None
    it = iter(values)
    root = TreeNode(next(it))
    queue = deque([root])
    while queue:
        node = queue.popleft()
        for side in ("left", "right"):
            v = next(it, None)
            if v is not None:
                child = TreeNode(v)
                setattr(node, side, child)
                queue.append(child)
    return root


def to_level(root):
    """Serialize a tree to LeetCode's level-order list, trailing Nones trimmed."""
    out = []
    queue = deque([root])
    while queue:
        node = queue.popleft()
        if node is None:
            out.append(None)
        else:
            out.append(node.val)
            queue.append(node.left)
            queue.append(node.right)
    while out and out[-1] is None:
        out.pop()
    return out
