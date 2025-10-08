class Solution:
    def restoreIpAddresses(self, s: str) -> list[str]:
        def backtrack(start=0, path=[]):
            if len(path) == 4 and start == len(s):
                result.append('.'.join(path))
                return
            if len(path) == 4 or start == len(s):
                return

            for length in range(1, 4):
                if start + length > len(s):
                    break
                segment = s[start:start + length]
                if (segment.startswith('0') and length > 1) or (length == 3 and int(segment) > 255):
                    continue
                backtrack(start + length, path + [segment])

        result = []
        backtrack()
        return result
    