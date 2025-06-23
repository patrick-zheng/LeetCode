function reverse(x: number): number {
  const INT_MIN = (-2) ** 31;
  const INT_MAX = 2 ** 31 - 1;

  let result = 0;
  const sign = x < 0 ? -1 : 1;
  let n = Math.abs(x);

  while (n !== 0) {
      const digit = n % 10;
      n = Math.floor(n / 10);

      result = result * 10 + digit;
  }

  result *= sign;

  if (result < INT_MIN || result > INT_MAX) {
      return 0;
  }

  return result;
};