# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Codec:
    def serialize(self, root: TreeNode | None) -> str:
        tokens: list[str] = []

        def dfs(node: TreeNode | None) -> None:
            if not node:
                tokens.append("N")
                return
            tokens.append(str(node.val))
            dfs(node.left)
            dfs(node.right)

        dfs(root)
        return ",".join(tokens)

    def deserialize(self, data: str) -> TreeNode | None:
        tokens = data.split(",")
        index = 0

        def dfs() -> TreeNode | None:
            nonlocal index
            if tokens[index] == "N":
                index += 1
                return None
            node = TreeNode(int(tokens[index]))
            index += 1
            node.left = dfs()
            node.right = dfs()
            return node

        return dfs()
