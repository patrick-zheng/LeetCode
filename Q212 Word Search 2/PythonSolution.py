class TrieNode:
    __slots__ = ("children", "word")

    def __init__(self) -> None:
        self.children: dict[str, TrieNode] = {}
        self.word: str | None = None


class Solution:
    def findWords(self, board: list[list[str]], words: list[str]) -> list[str]:
        root = TrieNode()
        for w in words:
            node = root
            for ch in w:
                node = node.children.setdefault(ch, TrieNode())
            node.word = w

        m, n = len(board), len(board[0])
        ans: list[str] = []

        def dfs(i: int, j: int, parent: TrieNode) -> None:
            if i < 0 or i >= m or j < 0 or j >= n:
                return
            letter = board[i][j]
            curr = parent.children.get(letter)
            if curr is None:
                return

            if curr.word is not None:
                ans.append(curr.word)
                curr.word = None

            board[i][j] = "#"
            dfs(i + 1, j, curr)
            dfs(i - 1, j, curr)
            dfs(i, j + 1, curr)
            dfs(i, j - 1, curr)
            board[i][j] = letter

            if not curr.children and curr.word is None:
                parent.children.pop(letter, None)

        for i in range(m):
            for j in range(n):
                dfs(i, j, root)

        return ans
