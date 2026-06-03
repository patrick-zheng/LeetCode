from collections import deque


class Solution:
    def numSquares(self, n: int) -> int:
        squares = [j * j for j in range(1, int(n**0.5) + 1)]
        queue = deque([n])
        visited = {n}
        steps = 0

        while queue:
            steps += 1
            for _ in range(len(queue)):
                value = queue.popleft()
                for sq in squares:
                    next_value = value - sq
                    if next_value == 0:
                        return steps
                    if next_value > 0 and next_value not in visited:
                        visited.add(next_value)
                        queue.append(next_value)
        return steps
