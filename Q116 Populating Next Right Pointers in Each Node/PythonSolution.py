# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left = None, right = None, next = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next

class Solution:
    def connect(self, root: Node | None) -> Node | None:
        if not root:
            return root

        leftmost = root
        while leftmost.left:
            cur = leftmost
            while cur:
                cur.left.next = cur.right # type: ignore
                if cur.next:
                    cur.right.next = cur.next.left # type: ignore
                cur = cur.next
            leftmost = leftmost.left
        return root
