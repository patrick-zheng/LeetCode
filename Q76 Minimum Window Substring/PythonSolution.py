from collections import Counter

class Solution:
    def minWindow(self, s: str, t: str) -> str:
        if not s or not t or len(t) > len(s):
            return ""

        need = Counter(t)
        required = len(need)
        window = Counter()
        formed = 0

        ans_len = float('inf')
        ans_l = ans_r = 0

        l = 0
        for r, ch in enumerate(s):
            window[ch] += 1
            if ch in need and window[ch] == need[ch]:
                formed += 1

            while formed == required:
                if (r - l + 1) < ans_len:
                    ans_len = r - l + 1
                    ans_l, ans_r = l, r

                left_ch = s[l]
                window[left_ch] -= 1
                if left_ch in need and window[left_ch] < need[left_ch]:
                    formed -= 1
                l += 1

        return "" if ans_len == float('inf') else s[ans_l:ans_r + 1]
    