class Solution:
    def combinationSum3(self, k: int, n: int) -> list[list[int]]:
        res: list[list[int]] = []
        path: list[int] = []

        def min_sum_k(start: int, kk: int) -> int:
            return kk * start + kk * (kk - 1) // 2

        def max_sum_k(start: int, kk: int) -> int:
            total = 0
            x = 9
            for _ in range(kk):
                if x < start:
                    return -1
                total += x
                x -= 1
            return total

        def dfs(start: int, k_left: int, target: int) -> None:
            if k_left == 0:
                if target == 0:
                    res.append(path.copy())
                return
            if start > 9 or target <= 0:
                return
            if 10 - start < k_left:
                return

            lo = min_sum_k(start, k_left)
            hi = max_sum_k(start, k_left)
            if hi < 0 or target < lo or target > hi:
                return

            for i in range(start, 10):
                if i > target:
                    break
                path.append(i)
                dfs(i + 1, k_left - 1, target - i)
                path.pop()

        dfs(1, k, n)
        return res
