# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []

class Solution:
    def cloneGraph(self, node: Node) -> Node:
        if not node:
            return None

        old_to_new = {}

        def dfs(curr: 'Node') -> 'Node':
            # If we already copied this node, return the copy
            if curr in old_to_new:
                return old_to_new[curr]

            # Create a new node and store it in the map
            copy = Node(curr.val)
            old_to_new[curr] = copy

            # Recursively clone neighbors
            for nei in curr.neighbors:
                copy.neighbors.append(dfs(nei))

            return copy

        return dfs(node)
    