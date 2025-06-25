class Solution:
    def maxArea(self, height: list[int]) -> int:
        left = 0
        right = len(height) - 1
        maxVolume = 0
        h = height  # Local alias for speed

        while left < right:
            hl = h[left]
            hr = h[right]

            if hl < hr:
                volume = hl * (right - left)
                left += 1
            else:
                volume = hr * (right - left)
                right -= 1

            if volume > maxVolume:
                maxVolume = volume

        return maxVolume
