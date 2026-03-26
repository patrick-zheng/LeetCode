use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut graph = vec![Vec::new(); n];
        let mut indegree = vec![0; n];

        for p in prerequisites {
            let a = p[0] as usize;
            let b = p[1] as usize;
            graph[b].push(a);
            indegree[a] += 1;
        }

        let mut q = VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                q.push_back(i);
            }
        }

        let mut taken = 0;

        while let Some(cur) = q.pop_front() {
            taken += 1;
            for &nxt in &graph[cur] {
                indegree[nxt] -= 1;
                if indegree[nxt] == 0 {
                    q.push_back(nxt);
                }
            }
        }

        taken == n
    }
}
