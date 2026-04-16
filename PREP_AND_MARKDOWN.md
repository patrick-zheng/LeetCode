# Prep checklist for each new LeetCode problem

Use this when you add another question folder so solutions stay consistent and
`Explanation.md` passes the usual markdownlint rules in this repo.

---

## 1. Folder and files

1. **Create a folder** named `Q{number} {Short Title}` (match existing style,
   e.g. `Q221 Maximal Square`).
2. **Add four files** (same names as other problems):
   - `PythonSolution.py` — `class Solution` and method names as on LeetCode.
   - `CppSolution.cpp` — `class Solution`, `vector`, LeetCode-style API.
   - `RustSolution.rs` — `pub struct Solution`, `impl Solution`,
     `pub fn snake_case(...)`.
   - `Explanation.md` — write-up (see section 2).

3. **Implement the solution you would submit** on LeetCode (time/space fit
   constraints), not a toy brute force unless the problem is trivial.

4. **Sanity-check code** before committing:
   - Python: run official samples plus a few edges.
   - C++: `g++ -std=c++17 -Wall -Wextra -c CppSolution.cpp`
   - Rust: `rustc --edition 2021 --crate-type lib RustSolution.rs` (no
     `main`, LeetCode-style).

---

## 2. `Explanation.md` structure

Mirror **Q220** / **Q221**:

- Title: `# LeetCode Problem: {Title}`
- **Problem Link** and **Solution Link** (problem URL + `/solutions/`).
- `---` between major sections.
- **`## Algorithm`** — idea, steps, formulas (`\[ ... \]` when helpful).
- **`## Constraints`** — bullets from the statement.
- **`## Complexity`** — two-column table (see section 3 for lint rules).

Keep prose clear; skip emoji unless you match an older folder on purpose.

---

## 3. Markdownlint checks for `Explanation.md`

Run editor diagnostics or:

```bash
npx --yes markdownlint-cli2 "Q### …/Explanation.md"
```

Fix what your config enforces. These rules already mattered here:

### MD060 — table column style (pipes must align)

- Each row needs the **same** inner widths between `|` so columns line up.
- **Match Q220’s complexity table**: first column **20** characters between
  pipes, second column **81**; separator `---` segments use the same lengths.
- Long text in column 2: **pad with trailing spaces** on header, separator,
  and every body row so the closing `|` aligns.

### MD052 — reference-link false positives

CommonMark treats **`[text][label]`** as a reference link. Index chains like
**`dp[i][j]`** or **`dp[i-1][j]`** trigger MD052 (“missing reference for
`j`”, etc.).

**Safer patterns:**

- In **LaTeX**, use subscripts: `dp_{i,j}`, `dp_{i-1,j}`, not `dp[i][j]`
  inside `\[ … \]` or `\( … \)`.
- In **inline code**, use comma indices: `` `dp[i, j]` ``,
  `` `matrix[i, j]` ``, or keep indices in math only.

After edits, search for **`][`** (closing bracket then opening bracket). Avoid
that pattern in plain text and math; code spans help but math is still scanned.

### MD013 — line length

With a typical **80**-column limit, long paragraphs and wide tables fail MD013.
Wrap prose; for the complexity table, either accept MD013 on those rows in
`Explanation.md` or widen the limit / disable MD013 for that path in
`.markdownlint-cli2.jsonc` (match team choice).

---

## 4. Complexity table template

Do **not** invent new column widths. Copy the **header** and **separator** lines
from `Q220 Contains Duplicate III/Explanation.md` under `## Complexity`, then
add `Time Complexity` / `Space Complexity` rows. Pad the second column on
every row to **81** characters between pipes (count with an editor column or a
small script). That keeps **MD060** satisfied.

---

## 5. Commit message expectation

Match **this repository’s history** (`git log`), not a generic open-source
template.

### Default for a finished problem folder

- **Subject:** `completed Q###` (lowercase `completed`, space, `Q` + number).
  Example: `completed Q220`.
- **Body (often used):** a blank line after the subject, then:
  `Made-with: Cursor`

### Other edits

- Use a short **imperative** subject that states the change, same style as
  `remove Q213 __pycache__, ignore Python bytecode` when the work is not
  “completed Q###”.

### Typos to avoid

- Use **`completed`**, not `competed` (see older history for Q197).

### If you want richer messages later

- You may still use a fuller subject (problem title, languages, fix reason) or
  a short body with algorithm notes; keep lines readable in `git log`
  (subject ~72 chars is a soft limit).

---

## 6. Before you call a problem “done”

- [ ] Folder name and four solution files present.
- [ ] Code matches LeetCode signatures; compiles / runs on samples.
- [ ] `Explanation.md`: algorithm, constraints, complexity table.
- [ ] No MD060 misalignment; no `[i][j]`-style chains in math (MD052).
- [ ] Lint / CI clean for the rules this repo enforces.
- [ ] Commit message follows **section 5** (`completed Q###` and optional
  `Made-with: Cursor`, or a clear imperative subject for non-problem commits).
