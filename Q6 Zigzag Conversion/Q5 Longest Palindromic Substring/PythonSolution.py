class Solution:
	def convert(self, s: str, numRows: int) -> str:
		if numRows == 1 or numRows >= len(s):
			return s

		rows: list[str] = [''] * numRows
		curr_row: int = 0
		going_down: bool = False

		for char in s:
			rows[curr_row] += char
			if curr_row == 0 or curr_row == numRows - 1:
				going_down = not going_down
			curr_row += 1 if going_down else -1

		return ''.join(rows)
