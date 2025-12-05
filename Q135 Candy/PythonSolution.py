class Solution:
    def candy(self, ratings: list[int]) -> int:
        n = len(ratings)
        if n == 0:
            return 0
        
        left = [1] * n
        right = [1] * n
        
        # Left to right: handle increasing sequences
        for i in range(1, n):
            if ratings[i] > ratings[i - 1]:
                left[i] = left[i - 1] + 1
        
        # Right to left: handle increasing sequences from the right
        for i in range(n - 2, -1, -1):
            if ratings[i] > ratings[i + 1]:
                right[i] = right[i + 1] + 1
        
        # Take max from both directions for each child
        total = 0
        for i in range(n):
            total += max(left[i], right[i])
        
        return total
