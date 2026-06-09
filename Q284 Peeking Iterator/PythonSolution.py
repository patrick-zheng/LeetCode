class PeekingIterator:
    def __init__(self, iterator):
        self._iterator = iterator
        self._has_next = iterator.hasNext()
        self._peeked = iterator.next() if self._has_next else 0

    def peek(self) -> int:
        return self._peeked

    def next(self) -> int:
        result = self._peeked
        self._has_next = self._iterator.hasNext()
        if self._has_next:
            self._peeked = self._iterator.next()
        return result

    def hasNext(self) -> bool:
        return self._has_next
