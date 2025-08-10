function groupAnagrams(strs: string[]): string[][] {
    const map = new Map<string, string[]>();

    for (const word of strs) {
        const count = new Array(26).fill(0);
        for (const char of word) {
            count[char.charCodeAt(0) - 'a'.charCodeAt(0)]++;
        }
        const key = count.join('#');  // Use '#' as separator to avoid ambiguity
        if (!map.has(key)) {
            map.set(key, []);
        }
        map.get(key)!.push(word);
    }

    return Array.from(map.values());
};
