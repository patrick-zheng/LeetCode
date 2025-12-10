# Definition for a Node.
class Node:
    def __init__(self, x: int, next: Node | None = None, random: Node | None = None):
        self.val = int(x)
        self.next = next
        self.random = random


class Solution:
    def copyRandomList(self, head: Node | None) -> Node | None:
        if not head:
            return None

        # 1) Interleave copied nodes
        curr = head
        while curr:
            copy = Node(curr.val)
            copy.next = curr.next
            curr.next = copy
            curr = copy.next

        # 2) Fix random pointers for copies
        curr = head
        while curr:
            copy = curr.next
            if curr.random:
                copy.random = curr.random.next
            curr = copy.next

        # 3) Detach copied list from original
        curr = head
        copy_head = head.next
        while curr:
            copy = curr.next
            curr.next = copy.next
            if copy.next:
                copy.next = copy.next.next
            curr = curr.next

        return copy_head
    