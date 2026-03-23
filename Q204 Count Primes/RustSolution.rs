pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        if n <= 2 {
            return 0;
        }

        let mut is_prime = vec![true; n];
        is_prime[0] = false;
        is_prime[1] = false;

        let mut p = 2;
        while p * p < n {
            if is_prime[p] {
                let mut multiple = p * p;
                while multiple < n {
                    is_prime[multiple] = false;
                    multiple += p;
                }
            }
            p += 1;
        }

        is_prime.iter().filter(|&&x| x).count() as i32
    }
}
