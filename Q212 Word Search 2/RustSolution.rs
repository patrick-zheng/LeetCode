#[derive(Default)]
struct TrieNode {
    children: [Option<usize>; 26],
    word: Option<String>,
}

pub struct Solution;

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        if board.is_empty() || board[0].is_empty() {
            return Vec::new();
        }

        let mut arena: Vec<TrieNode> = vec![TrieNode::default()];
        for w in &words {
            let mut node = 0usize;
            for ch in w.chars() {
                let c = (ch as u8 - b'a') as usize;
                node = match arena[node].children[c] {
                    Some(n) => n,
                    None => {
                        let id = arena.len();
                        arena.push(TrieNode::default());
                        arena[node].children[c] = Some(id);
                        id
                    }
                };
            }
            arena[node].word = Some(w.clone());
        }

        let m = board.len();
        let n = board[0].len();
        let mut ans = Vec::new();
        for i in 0..m {
            for j in 0..n {
                Self::dfs(&mut board, i, j, 0, &mut arena, &mut ans, m, n);
            }
        }
        ans
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        i: usize,
        j: usize,
        parent: usize,
        arena: &mut Vec<TrieNode>,
        ans: &mut Vec<String>,
        m: usize,
        n: usize,
    ) {
        let ch = board[i][j];
        if ch == '#' {
            return;
        }
        let c = (ch as u8 - b'a') as usize;
        let Some(curr) = arena[parent].children[c] else {
            return;
        };

        if let Some(w) = arena[curr].word.take() {
            ans.push(w);
        }

        board[i][j] = '#';
        if i > 0 {
            Self::dfs(board, i - 1, j, curr, arena, ans, m, n);
        }
        if i + 1 < m {
            Self::dfs(board, i + 1, j, curr, arena, ans, m, n);
        }
        if j > 0 {
            Self::dfs(board, i, j - 1, curr, arena, ans, m, n);
        }
        if j + 1 < n {
            Self::dfs(board, i, j + 1, curr, arena, ans, m, n);
        }
        board[i][j] = ch;

        let leaf = arena[curr].children.iter().all(|x| x.is_none());
        if leaf && arena[curr].word.is_none() {
            arena[parent].children[c] = None;
        }
    }
}
