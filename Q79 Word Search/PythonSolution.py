from collections import Counter

class Solution:
    def exist(self, board: list[list[str]], word: str) -> bool:
        if not board or not board[0]:
            return False

        R, C = len(board), len(board[0])

        # --- Optimization 1: letter-count pruning ---
        # If the board doesn't contain enough of any required letter, fail early.
        flat = [ch for row in board for ch in row]
        board_count = Counter(flat)
        word_count = Counter(word)
        if any(board_count[ch] < cnt for ch, cnt in word_count.items()):
            return False

        # --- Optimization 2: search from rarer end ---
        # If the last char is rarer than the first, search the reversed word.
        # (This reduces branching at the start.)
        if board_count[word[0]] > board_count[word[-1]]:
            word = word[::-1]

        dirs = ((1, 0), (-1, 0), (0, 1), (0, -1))

        def dfs(r: int, c: int, i: int) -> bool:
            # i = index in word we are trying to match at (r, c)
            if i == len(word) - 1:
                return board[r][c] == word[i]

            if board[r][c] != word[i]:
                return False

            # mark visited in-place
            tmp = board[r][c]
            board[r][c] = "#"  # sentinel not present in input

            nxt = i + 1
            for dr, dc in dirs:
                nr, nc = r + dr, c + dc
                if 0 <= nr < R and 0 <= nc < C and board[nr][nc] != "#":
                    if dfs(nr, nc, nxt):
                        board[r][c] = tmp
                        return True

            # restore
            board[r][c] = tmp
            return False

        # Try only cells matching the first letter to minimize calls
        first = word[0]
        for r in range(R):
            row = board[r]
            for c in range(C):
                if row[c] == first and dfs(r, c, 0):
                    return True

        return False
