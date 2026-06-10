class Solution:
    def gameOfLife(self, board: list[list[int]]) -> None:
        rows = len(board)
        cols = len(board[0])

        for r in range(rows):
            for c in range(cols):
                live_neighbors = 0
                for nr in range(max(0, r - 1), min(rows, r + 2)):
                    for nc in range(max(0, c - 1), min(cols, c + 2)):
                        if nr == r and nc == c:
                            continue
                        if board[nr][nc] in (1, 2):
                            live_neighbors += 1

                if board[r][c] == 1 and (live_neighbors < 2 or live_neighbors > 3):
                    board[r][c] = 2
                elif board[r][c] == 0 and live_neighbors == 3:
                    board[r][c] = 3

        for r in range(rows):
            for c in range(cols):
                if board[r][c] == 2:
                    board[r][c] = 0
                elif board[r][c] == 3:
                    board[r][c] = 1
