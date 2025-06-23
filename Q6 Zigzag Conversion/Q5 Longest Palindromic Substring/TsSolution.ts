function convert(s: string, numRows: number): string {
  if (numRows === 1 || numRows >= s.length) {
    return s;
  }

  const rows: string[] = Array(numRows).fill('');
  let currRow: number = 0;
  let goingDown: boolean = false;

  for (const char of s) {
    rows[currRow] += char;
    if (currRow === 0 || currRow === numRows - 1) {
      goingDown = !goingDown;
    }
    currRow += goingDown ? 1 : -1;
  }
  return rows.join('');
};