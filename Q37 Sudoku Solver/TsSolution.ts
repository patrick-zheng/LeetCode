/**
 Do not return anything, modify board in-place instead.
 */
function solveSudoku(board: string[][]): void {
    const rows = Array.from({ length: 9 }, () => new Set<string>());
    const cols = Array.from({ length: 9 }, () => new Set<string>());
    const boxes = Array.from({ length: 9 }, () => new Set<string>());
    const empty: [number, number][] = [];

    // Preprocess
    for (let r = 0; r < 9; r++) {
        for (let c = 0; c < 9; c++) {
        const val = board[r][c];
        if (val === ".") {
            empty.push([r, c]);
        } else {
            rows[r].add(val);
            cols[c].add(val);
            boxes[Math.floor(r / 3) * 3 + Math.floor(c / 3)].add(val);
        }
        }
    }

    const solve = (idx: number): boolean => {
        if (idx === empty.length) return true;

        const [r, c] = empty[idx];
        const b = Math.floor(r / 3) * 3 + Math.floor(c / 3);

        for (let d = 1; d <= 9; d++) {
        const digit = d.toString();
        if (
            !rows[r].has(digit) &&
            !cols[c].has(digit) &&
            !boxes[b].has(digit)
        ) {
            board[r][c] = digit;
            rows[r].add(digit);
            cols[c].add(digit);
            boxes[b].add(digit);

            if (solve(idx + 1)) return true;

            board[r][c] = ".";
            rows[r].delete(digit);
            cols[c].delete(digit);
            boxes[b].delete(digit);
        }
        }

        return false;
    };

    solve(0);
};