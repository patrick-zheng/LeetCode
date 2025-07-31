function multiply(num1: string, num2: string): string {
    if (num1 === "0" || num2 === "0") return "0";

    const len1 = num1.length;
    const len2 = num2.length;
    const result = new Array(len1 + len2).fill(0);

    for (let i = len1 - 1; i >= 0; i--) {
        for (let j = len2 - 1; j >= 0; j--) {
            const mul = (num1.charCodeAt(i) - 48) * (num2.charCodeAt(j) - 48);
            const p1 = i + j, p2 = i + j + 1;
            const total = mul + result[p2];

            result[p2] = total % 10;
            result[p1] += Math.floor(total / 10);
        }
    }

    while (result[0] === 0) result.shift();
    return result.join('');
};
