function countAndSay(n: number): string {
    if (n === 1) return "1";

    const prev = countAndSay(n - 1);
    let result = "";
    let count = 1;

    for (let i = 1; i < prev.length; i++) {
        if (prev[i] === prev[i - 1]) {
            count++;
        } else {
            result += count.toString() + prev[i - 1];
            count = 1;
        }
    }

    result += count.toString() + prev[prev.length - 1];
    return result;
};
