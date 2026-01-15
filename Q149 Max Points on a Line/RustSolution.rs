use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 {
            return n as i32;
        }

        let mut ans: i32 = 2;

        for i in 0..n {
            // Early stop: can't beat current answer with remaining anchors
            if ans as usize >= n - i {
                break;
            }

            let x1 = points[i][0];
            let y1 = points[i][1];

            let mut map: HashMap<(i32, i32), i32> = HashMap::new();
            let mut dup: i32 = 0;
            let mut best: i32 = 0;

            for j in (i + 1)..n {
                let mut dx = points[j][0] - x1;
                let mut dy = points[j][1] - y1;

                if dx == 0 && dy == 0 {
                    dup += 1;
                    continue;
                }

                // Normalize slope
                let key: (i32, i32) = if dx == 0 {
                    (1, 0) // vertical
                } else if dy == 0 {
                    (0, 1) // horizontal
                } else {
                    let g = Solution::gcd(dy, dx);
                    dy /= g;
                    dx /= g;

                    // normalize sign: force dx > 0
                    if dx < 0 {
                        dx = -dx;
                        dy = -dy;
                    }
                    (dy, dx)
                };

                let entry = map.entry(key).or_insert(0);
                *entry += 1;
                if *entry > best {
                    best = *entry;
                }
            }

            ans = ans.max(best + dup + 1);
        }

        ans
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }
}
