pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total = 0; // net gas over all stations
        let mut tank = 0;  // gas in tank during current simulation
        let mut start = 0; // candidate starting index

        for i in 0..gas.len() {
            let diff = gas[i] - cost[i];
            total += diff;
            tank += diff;

            // If we can't reach the next station from current start,
            // move start to i + 1 and reset tank.
            if tank < 0 {
                start = (i + 1) as i32;
                tank = 0;
            }
        }

        if total >= 0 {
            start
        } else {
            -1
        }
    }
}
