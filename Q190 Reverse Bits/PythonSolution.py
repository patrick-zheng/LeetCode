class Solution:
    _rev8 = [int(f"{i:08b}"[::-1], 2) for i in range(256)]
    
    def reverseBits(self, n: int) -> int:
        res = 0
        for _ in range(32):
            res = (res << 1) | (n & 1)
            n >>= 1
        return res
    