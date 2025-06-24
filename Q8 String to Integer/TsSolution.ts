function myAtoi(s: string): number {
    const INT_MAX = 2147483647;
    const INT_MIN = -2147483648;
    s = s.trim();
    if (s.length === 0) return 0;

    let sign: number = 1;
    let result: number = 0;
    let counter: number = 0;
    if (s[0] === '-') {
        sign = -1;
        counter++;
    } else if (s[0] === '+') {
        counter++;
    }

    while (counter < s.length && s[counter] >= '0' && s[counter] <= '9') {
        const digit = Number(s[counter])
        result = result * 10 + digit;
        counter++;
    }

    return Math.max(INT_MIN, Math.min(INT_MAX, result * sign));
};
