function combinationSum2(candidates: number[], target: number): number[][] {
    candidates.sort((a, b) => a - b);
    const result: number[][] = [];

    const backtrack = (start: number, path: number[], total: number) => {
        if (total === target) {
            result.push([...path]);
            return;
        }

        for (let i = start; i < candidates.length; i++) {
            if (i > start && candidates[i] === candidates[i - 1]) continue; // Skip duplicates
            if (total + candidates[i] > target) break;

            path.push(candidates[i]);
            backtrack(i + 1, path, total + candidates[i]);
            path.pop();
        }
    };

    backtrack(0, [], 0);
    return result;
};
