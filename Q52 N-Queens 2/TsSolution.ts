function totalNQueens(n: number): number {
    // bitmasks for columns, diag1 (top-left→bottom-right), diag2 (top-right→bottom-left)
    const limit = (1 << n) - 1;

    const dfs = (cols: number, d1: number, d2: number): number => {
        if (cols === limit) return 1; // placed queens in all rows
        let count = 0;
        // positions available in current row
        let avail = ~(cols | d1 | d2) & limit;
        while (avail) {
            // pick least significant 1-bit
            const p = avail & -avail;
            avail -= p;
            count += dfs(cols | p, (d1 | p) << 1 & limit, (d2 | p) >> 1);
        }
        return count;
    };

    return dfs(0, 0, 0);
};
