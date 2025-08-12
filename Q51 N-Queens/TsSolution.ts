function solveNQueens(n: number): string[][] {
    const result: string[][] = [];
    const board: string[][] = Array.from({ length: n }, () => Array(n).fill('.'));

    function backtrack(row: number, cols: number, diag1: number, diag2: number) {
        if (row === n) {
            result.push(board.map(r => r.join('')));
            return;
        }

        let available = ((1 << n) - 1) & ~(cols | diag1 | diag2);
        while (available) {
            let bit = available & -available;
            available -= bit;
            let col = Math.log2(bit) | 0;

            board[row][col] = 'Q';
            backtrack(row + 1, cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1);
            board[row][col] = '.';
        }
    }

    backtrack(0, 0, 0, 0);
    return result;
};
