class Solution:
    def diffWaysToCompute(self, expression: str) -> list[int]:
        memo: dict[str, list[int]] = {}

        def dfs(s: str) -> list[int]:
            if s in memo:
                return memo[s]
            if s.isdigit():
                memo[s] = [int(s)]
                return memo[s]
            out: list[int] = []
            for i, ch in enumerate(s):
                if ch not in "+-*":
                    continue
                left_vals = dfs(s[:i])
                right_vals = dfs(s[i + 1 :])
                for a in left_vals:
                    for b in right_vals:
                        if ch == "+":
                            out.append(a + b)
                        elif ch == "-":
                            out.append(a - b)
                        else:
                            out.append(a * b)
            memo[s] = out
            return out

        return dfs(expression)
