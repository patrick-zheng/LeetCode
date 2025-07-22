class Solution:
    def solveSudoku(self, board: list[list[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        from collections import defaultdict

        # Track available numbers for rows, cols, and boxes
        rows = [set(str(i) for i in range(1, 10)) for _ in range(9)]
        cols = [set(str(i) for i in range(1, 10)) for _ in range(9)]
        boxes = [set(str(i) for i in range(1, 10)) for _ in range(9)]

        empty = []

        # Preprocess board
        for r in range(9):
            for c in range(9):
                val = board[r][c]
                if val == '.':
                    empty.append((r, c))
                else:
                    rows[r].remove(val)
                    cols[c].remove(val)
                    boxes[(r // 3) * 3 + (c // 3)].remove(val)

        def backtrack(i):
            if i == len(empty):
                return True  # Solved all

            r, c = empty[i]
            b = (r // 3) * 3 + (c // 3)
            choices = rows[r] & cols[c] & boxes[b]
            for num in choices:
                board[r][c] = num
                rows[r].remove(num)
                cols[c].remove(num)
                boxes[b].remove(num)

                if backtrack(i + 1):
                    return True

                # Undo
                board[r][c] = '.'
                rows[r].add(num)
                cols[c].add(num)
                boxes[b].add(num)

            return False

        backtrack(0)
        