pub struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let secret = secret.as_bytes();
        let guess = guess.as_bytes();
        let mut bulls = 0i32;
        let mut secret_freq = [0i32; 10];
        let mut guess_freq = [0i32; 10];

        for i in 0..secret.len() {
            if secret[i] == guess[i] {
                bulls += 1;
            } else {
                secret_freq[(secret[i] - b'0') as usize] += 1;
                guess_freq[(guess[i] - b'0') as usize] += 1;
            }
        }

        let cows: i32 = (0..10)
            .map(|d| secret_freq[d].min(guess_freq[d]))
            .sum();
        format!("{bulls}A{cows}B")
    }
}
