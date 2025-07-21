function isValidSudoku(board: string[][]): boolean {
    const rows: Set<string>[] = Array.from({ length: 9 }, () => new Set());
    const cols: Set<string>[] = Array.from({ length: 9 }, () => new Set());
    const boxes: Set<string>[] = Array.from({ length: 9 }, () => new Set());

    for (let r = 0; r < 9; r++) {
        for (let c = 0; c < 9; c++) {
            const num = board[r][c];
            if (num === '.') continue;

            const boxIndex = Math.floor(r / 3) * 3 + Math.floor(c / 3);
            if (rows[r].has(num) || cols[c].has(num) || boxes[boxIndex].has(num)) {
                return false;
            }

            rows[r].add(num);
            cols[c].add(num);
            boxes[boxIndex].add(num);
        }
    }

    return true;
};
