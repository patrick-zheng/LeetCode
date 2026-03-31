use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut graph = vec![Vec::<usize>::new(); n];
        let mut indegree = vec![0usize; n];

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

        let mut order = Vec::with_capacity(n);

        while let Some(u) = q.pop_front() {
            order.push(u as i32);
            for &v in &graph[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        if order.len() == n { order } else { vec![] }
    }
}
