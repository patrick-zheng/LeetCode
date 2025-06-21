function longestPalindrome(s: string): string {
  if (s.length === 0) return "";

  // Step 1: Preprocess the string
  const preprocess = (s: string): string => {
    return "^#" + s.split("").join("#") + "#$";
  };

  const t = preprocess(s);
  const n = t.length;
  const P = new Array(n).fill(0);
  let center = 0, right = 0;

  // Step 2: Manacher's Algorithm
  for (let i = 1; i < n - 1; i++) {
    const mirror = 2 * center - i;

    if (i < right) {
      P[i] = Math.min(right - i, P[mirror]);
    }

    // Expand palindrome centered at i
    while (t[i + P[i] + 1] === t[i - P[i] - 1]) {
      P[i]++;
    }

    // Update center and right boundary
    if (i + P[i] > right) {
      center = i;
      right = i + P[i];
    }
  }

  // Step 3: Find the longest palindrome
  let maxLen = 0;
  let centerIndex = 0;
  for (let i = 1; i < n - 1; i++) {
    if (P[i] > maxLen) {
      maxLen = P[i];
      centerIndex = i;
    }
  }

  const start = (centerIndex - maxLen) / 2;
  return s.substring(start, start + maxLen);
}

