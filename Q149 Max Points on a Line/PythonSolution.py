from math import gcd

class Solution:
    def maxPoints(self, points: list[list[int]]) -> int:
        n = len(points)
        if n <= 2:
            return n

        ans = 2

        for i in range(n):
            # Early stop: even if all remaining points line up with points[i], can't beat ans
            if ans >= n - i:
                break

            x1, y1 = points[i]
            slopes = {}  # (dy, dx) -> count
            dup = 0
            best = 0

            for j in range(i + 1, n):
                x2, y2 = points[j]
                dx = x2 - x1
                dy = y2 - y1

                if dx == 0 and dy == 0:
                    dup += 1
                    continue

                if dx == 0:
                    key = (1, 0)          # vertical
                elif dy == 0:
                    key = (0, 1)          # horizontal
                else:
                    g = gcd(dy if dy > 0 else -dy, dx if dx > 0 else -dx)
                    dy //= g
                    dx //= g

                    # normalize sign: force dx > 0
                    if dx < 0:
                        dx = -dx
                        dy = -dy

                    key = (dy, dx)

                c = slopes.get(key, 0) + 1
                slopes[key] = c
                if c > best:
                    best = c

            # best counts points besides anchor; add anchor + duplicates
            total = best + dup + 1
            if total > ans:
                ans = total

        return ans
    