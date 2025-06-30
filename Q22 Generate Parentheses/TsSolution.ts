function generateParenthesis(n: number): string[] {
    const result: string[] = [];

    function backtrack(current: string, openCount: number, closeCount: number): void {
        if (current.length === 2 * n) {
            result.push(current);
            return;
        }
        if (openCount < n) {
            backtrack(current + '(', openCount + 1, closeCount);
        }
        if (closeCount < openCount) {
            backtrack(current + ')', openCount, closeCount + 1);
        }
    }

    backtrack("", 0, 0);
    return result;
};
