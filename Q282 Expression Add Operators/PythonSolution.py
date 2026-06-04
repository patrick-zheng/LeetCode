class Solution:
    def addOperators(self, num: str, target: int) -> list[str]:
        n = len(num)
        result: list[str] = []
        path: list[str] = []

        def try_push(
            value: int,
            next_index: int,
            curr_val: int,
            last_operand: int,
            started: bool,
        ) -> None:
            value_str = str(value)
            if not started:
                mark = len(path)
                path.extend(value_str)
                dfs(next_index, value, value, True)
                del path[mark:]
                return

            for op, new_val, new_last in (
                ("+", curr_val + value, value),
                ("-", curr_val - value, -value),
                (
                    "*",
                    curr_val - last_operand + last_operand * value,
                    last_operand * value,
                ),
            ):
                mark = len(path)
                path.append(op)
                path.extend(value_str)
                dfs(next_index, new_val, new_last, True)
                del path[mark:]

        def dfs(index: int, curr_val: int, last_operand: int, started: bool) -> None:
            if index == n:
                if curr_val == target:
                    result.append("".join(path))
                return

            if num[index] == "0":
                try_push(0, index + 1, curr_val, last_operand, started)
                return

            value = 0
            for end in range(index, n):
                if end > index and num[index] == "0":
                    break
                value = value * 10 + ord(num[end]) - 48
                try_push(value, end + 1, curr_val, last_operand, started)

        dfs(0, 0, 0, False)
        return result
