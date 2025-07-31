function isMatch(s: string, p: string): boolean {
    const m = p.length + 1;
    const n = s.length + 1;
    const dp: boolean[][] = Array.from({ length: m }, () => new Array(n).fill(false));

    dp[0][0] = true;

    for (let i = 1; i < m; i++) {
        if (p[i - 1] === '*') {
            dp[i][0] = dp[i - 1][0];
        }
    }

    for (let i = 1; i < m; i++) {
        for (let j = 1; j < n; j++) {
            const pc = p[i - 1], sc = s[j - 1];
            if (pc === '?') {
                dp[i][j] = dp[i - 1][j - 1];
            } else if (pc === '*') {
                dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
            } else {
                dp[i][j] = pc === sc && dp[i - 1][j - 1];
            }
        }
    }

    return dp[m - 1][n - 1];
}
