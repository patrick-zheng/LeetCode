use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut events: Vec<(i32, i32)> = Vec::with_capacity(buildings.len() * 2);
        for b in buildings {
            let l = b[0];
            let r = b[1];
            let h = b[2];
            events.push((l, -h));
            events.push((r, h));
        }
        events.sort_unstable();

        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
        cnt.insert(0, 1);

        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut i = 0;
        while i < events.len() {
            let x = events[i].0;
            while i < events.len() && events[i].0 == x {
                let h = events[i].1;
                if h < 0 {
                    let height = -h;
                    *cnt.entry(height).or_insert(0) += 1;
                } else {
                    let height = h;
                    if let Some(c) = cnt.get_mut(&height) {
                        *c -= 1;
                        if *c == 0 {
                            cnt.remove(&height);
                        }
                    }
                }
                i += 1;
            }
            let max_h = *cnt.iter().next_back().unwrap().0;
            if res.is_empty() || res.last().unwrap()[1] != max_h {
                res.push(vec![x, max_h]);
            }
        }
        res
    }
}
