pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as u32;
        let limit: u32 = (1u32 << n) - 1;

        fn dfs(cols: u32, d1: u32, d2: u32, limit: u32) -> i32 {
            if cols == limit { return 1; }
            let mut count = 0;
            let mut avail = !(cols | d1 | d2) & limit;
            while avail != 0 {
                let p = avail & (!avail + 1); // lowest set bit
                avail ^= p;
                count += dfs(
                    cols | p,
                    ((d1 | p) << 1) & limit,
                    (d2 | p) >> 1,
                    limit
                );
            }
            count
        }

        dfs(0, 0, 0, limit)
    }
}
