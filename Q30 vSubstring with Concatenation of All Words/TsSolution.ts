function findSubstring(s: string, words: string[]): number[] {
    if (words.length === 0 || s.length === 0) return [];

    const wordLen = words[0].length;
    const totalLen = wordLen * words.length;
    const wordCount = new Map<string, number>();
    const result: number[] = [];

    for (const word of words) {
        wordCount.set(word, (wordCount.get(word) || 0) + 1);
    }

    for (let i = 0; i < wordLen; i++) {
        let left = i;
        let right = i;
        const windowCount = new Map<string, number>();
        let count = 0;

        while (right + wordLen <= s.length) {
        const word = s.slice(right, right + wordLen);
        right += wordLen;

        if (wordCount.has(word)) {
            windowCount.set(word, (windowCount.get(word) || 0) + 1);
            count++;

            while (windowCount.get(word)! > wordCount.get(word)!) {
            const leftWord = s.slice(left, left + wordLen);
            windowCount.set(leftWord, windowCount.get(leftWord)! - 1);
            left += wordLen;
            count--;
            }

            if (count === words.length) {
            result.push(left);
            }
        } else {
            windowCount.clear();
            count = 0;
            left = right;
        }
        }
    }

    return result;
};
