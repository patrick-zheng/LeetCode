function combinationSum(candidates: number[], target: number): number[][] {
    const result: number[][] = [];

    function backtrack(start: number, path: number[], remaining: number) {
        if (remaining === 0) {
            result.push([...path]);
            return;
        }
        if (remaining < 0) return;

        for (let i = start; i < candidates.length; i++) {
            path.push(candidates[i]);
            backtrack(i, path, remaining - candidates[i]);
            path.pop(); // backtrack
        }
    }

    backtrack(0, [], target);
    return result;
};
