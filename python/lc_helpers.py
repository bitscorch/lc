"""Local helpers for LeetCode solutions that use custom types (linked lists,
binary trees). LeetCode's judge defines ListNode/TreeNode itself, so `lc submit`
strips these imports — they exist only so solutions run and tests pass locally."""

from __future__ import annotations

from collections import deque


class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None) -> None:
        self.val = val
        self.next = next


class TreeNode:
    def __init__(
        self,
        val: int = 0,
        left: TreeNode | None = None,
        right: TreeNode | None = None,
    ) -> None:
        self.val = val
        self.left = left
        self.right = right


def list_of(values: list[int]) -> ListNode | None:
    """Build a linked list from a list (head = first element)."""
    head: ListNode | None = None
    for v in reversed(values):
        head = ListNode(v, head)
    return head


def to_list(node: ListNode | None) -> list[int]:
    """Collect a linked list back into a plain list."""
    out: list[int] = []
    while node:
        out.append(node.val)
        node = node.next
    return out


def cyclic_list(values: list[int], pos: int) -> ListNode | None:
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


def tree_of(values: list[int | None]) -> TreeNode | None:
    """Build a tree from LeetCode's level-order list (None = a missing node)."""
    if not values or values[0] is None:
        return None
    it = iter(values)
    first = next(it)
    assert first is not None  # guaranteed by the guard above
    root = TreeNode(first)
    queue: deque[TreeNode] = deque([root])
    while queue:
        node = queue.popleft()
        for side in ("left", "right"):
            v = next(it, None)
            if v is not None:
                child = TreeNode(v)
                setattr(node, side, child)
                queue.append(child)
    return root


def to_level(root: TreeNode | None) -> list[int | None]:
    """Serialize a tree to LeetCode's level-order list, trailing Nones trimmed."""
    out: list[int | None] = []
    queue: deque[TreeNode | None] = deque([root])
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


class Node:
    def __init__(self, val: int = 0, neighbors: list[Node] | None = None) -> None:
        self.val = val
        self.neighbors: list[Node] = neighbors if neighbors is not None else []


def graph_of(adj_list: list[list[int]]) -> Node | None:
    """Build an undirected graph from LeetCode's adjacency list (adj_list[i] =
    neighbor values of the node with val i+1). Returns the node with val 1, or
    None for an empty graph."""
    if not adj_list:
        return None
    nodes: dict[int, Node] = {i: Node(i) for i in range(1, len(adj_list) + 1)}
    for i, neighbors in enumerate(adj_list, start=1):
        nodes[i].neighbors = [nodes[j] for j in neighbors]
    return nodes[1]


def to_adj(node: Node | None) -> list[list[int]]:
    """Serialize a graph back to LeetCode's adjacency list (adj[i] = sorted
    neighbor vals of node i+1), for comparing a cloned graph to the expected."""
    if node is None:
        return []
    seen: dict[int, Node] = {}
    stack = [node]
    while stack:
        cur = stack.pop()
        if cur.val in seen:
            continue
        seen[cur.val] = cur
        stack.extend(nb for nb in cur.neighbors if nb.val not in seen)
    return [sorted(nb.val for nb in seen[v].neighbors) for v in range(1, max(seen) + 1)]


def is_deep_copy(original: Node | None, clone: Node | None) -> bool:
    """True if `clone` shares no Node objects with `original` — i.e. a real deep
    copy, not the original graph handed back unchanged."""

    def ids(node: Node | None) -> set[int]:
        seen: set[int] = set()
        stack: list[Node] = [node] if node else []
        while stack:
            cur = stack.pop()
            if id(cur) in seen:
                continue
            seen.add(id(cur))
            stack.extend(nb for nb in cur.neighbors if id(nb) not in seen)
        return seen

    return ids(original).isdisjoint(ids(clone))
