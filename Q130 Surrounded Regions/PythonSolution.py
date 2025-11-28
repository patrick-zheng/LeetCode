from collections import deque


class Solution:
    def solve(self, board: list[list[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        if not board or not board[0]:
            return

        m, n = len(board), len(board[0])

        def bfs(r: int, c: int) -> None:
            queue = deque()
            queue.append((r, c))
            board[r][c] = 'E'  # mark as escaped (non-captured)

            while queue:
                x, y = queue.popleft()
                for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                    nx, ny = x + dx, y + dy
                    if 0 <= nx < m and 0 <= ny < n and board[nx][ny] == 'O':
                        board[nx][ny] = 'E'
                        queue.append((nx, ny))

        # 1. Mark all 'O's connected to the borders as 'E'
        for i in range(m):
            if board[i][0] == 'O':
                bfs(i, 0)
            if board[i][n - 1] == 'O':
                bfs(i, n - 1)

        for j in range(n):
            if board[0][j] == 'O':
                bfs(0, j)
            if board[m - 1][j] == 'O':
                bfs(m - 1, j)

        # 2. Flip captured 'O's to 'X', and restore escaped 'E's to 'O'
        for i in range(m):
            for j in range(n):
                if board[i][j] == 'O':
                    board[i][j] = 'X'
                elif board[i][j] == 'E':
                    board[i][j] = 'O'
                    