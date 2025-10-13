pub struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut g = vec![0i64; n + 1];
        g[0] = 1;
        if n >= 1 { g[1] = 1; }
        for nodes in 2..=n {
            let mut total = 0i64;
            for root in 1..=nodes {
                total += g[root - 1] * g[nodes - root];
            }
            g[nodes] = total;
        }
        g[n] as i32
    }
}
