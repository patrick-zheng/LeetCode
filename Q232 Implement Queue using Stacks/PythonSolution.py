class MyQueue:
    def __init__(self):
        self._in: list[int] = []
        self._out: list[int] = []

    def push(self, x: int) -> None:
        self._in.append(x)

    def pop(self) -> int:
        self._move()
        return self._out.pop()

    def peek(self) -> int:
        self._move()
        return self._out[-1]

    def empty(self) -> bool:
        return not self._in and not self._out

    def _move(self) -> None:
        if self._out:
            return
        while self._in:
            self._out.append(self._in.pop())
