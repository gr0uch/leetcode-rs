impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut memo = vec![vec![0; (target + 1) as usize]; (n + 1) as usize];
        memo[0][0] = 1;
        for i_n in 1..(n + 1) as usize {
            for i_t in 1..=(target as usize) {
                for i_k in 1..=(k as usize) {
                    if i_k <= i_t {
                        memo[i_n][i_t] = (memo[i_n][i_t] + memo[i_n - 1][i_t - i_k]) % 1000000007;
                    }
                }
            }
        }
        memo[n as usize][target as usize]
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::num_rolls_to_target(2, 6, 7));
}
