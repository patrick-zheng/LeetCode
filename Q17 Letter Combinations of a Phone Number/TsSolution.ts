function letterCombinations(digits: string): string[] {
    if (!digits) return [];

    const digitToLetters: Record<string, string[]> = {
        '2': ['a', 'b', 'c'],
        '3': ['d', 'e', 'f'],
        '4': ['g', 'h', 'i'],
        '5': ['j', 'k', 'l'],
        '6': ['m', 'n', 'o'],
        '7': ['p', 'q', 'r', 's'],
        '8': ['t', 'u', 'v'],
        '9': ['w', 'x', 'y', 'z']
    };

    const result: string[] = [];

    function backtrack(index: number, path: string[]): void {
        if (index === digits.length) {
            result.push(path.join(''));
            return;
        }

        const currentDigit = digits[index];
        for (const letter of digitToLetters[currentDigit]) {
            path.push(letter);
            backtrack(index + 1, path);
            path.pop();
        }
    }

    backtrack(0, []);
    return result;
};
