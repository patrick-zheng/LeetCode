class Solution:
    def removeInvalidParentheses(self, s: str) -> list[str]:
        left_remove = 0
        right_remove = 0
        for ch in s:
            if ch == "(":
                left_remove += 1
            elif ch == ")":
                if left_remove > 0:
                    left_remove -= 1
                else:
                    right_remove += 1

        result: set[str] = set()
        path: list[str] = []

        def dfs(
            index: int,
            open_count: int,
            close_count: int,
            left_rem: int,
            right_rem: int,
        ) -> None:
            if index == len(s):
                if left_rem == 0 and right_rem == 0:
                    result.add("".join(path))
                return

            ch = s[index]
            if ch not in "()":
                path.append(ch)
                dfs(index + 1, open_count, close_count, left_rem, right_rem)
                path.pop()
                return

            if ch == "(" and left_rem > 0:
                dfs(index + 1, open_count, close_count, left_rem - 1, right_rem)
            elif ch == ")" and right_rem > 0:
                dfs(index + 1, open_count, close_count, left_rem, right_rem - 1)

            if ch == "(":
                path.append(ch)
                dfs(index + 1, open_count + 1, close_count, left_rem, right_rem)
                path.pop()
            elif close_count < open_count:
                path.append(ch)
                dfs(index + 1, open_count, close_count + 1, left_rem, right_rem)
                path.pop()

        dfs(0, 0, 0, left_remove, right_remove)
        return list(result)
