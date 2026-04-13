import heapq
from collections import defaultdict


class Solution:
    def getSkyline(self, buildings: list[list[int]]) -> list[list[int]]:
        # Sweep line: (x, h). Starts use negative h so sort places all starts before
        # ends at the same x, and taller starts before shorter ones.
        events: list[tuple[int, int]] = []
        for left, right, height in buildings:
            events.append((left, -height))
            events.append((right, height))
        events.sort()

        # Max-heap via min-heap of negative heights; lazy removal counts.
        max_heap = [0]
        removed: defaultdict[int, int] = defaultdict(int)

        def prune() -> None:
            while max_heap and removed[-max_heap[0]] > 0:
                removed[-max_heap[0]] -= 1
                heapq.heappop(max_heap)

        result: list[list[int]] = []
        i = 0
        n = len(events)
        while i < n:
            x = events[i][0]
            while i < n and events[i][0] == x:
                _, h = events[i]
                if h < 0:
                    heapq.heappush(max_heap, h)
                else:
                    removed[h] += 1
                i += 1
            prune()
            max_h = -max_heap[0]
            if not result or result[-1][1] != max_h:
                result.append([x, max_h])

        return result
