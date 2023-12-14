struct Solution;

impl Solution {
    /// "binary search" approach
    /// O(logn)
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let result: f64;
        let abs_n = (n as i64).abs(); // kringe

        if abs_n % 2 == 0 {
            result = Solution::my_pow(x * x, (abs_n / 2) as i32);
        } else {
            result = Solution::my_pow(x * x, ((abs_n - 1) / 2) as i32) * x;
        }

        if n < 0 {
            return 1.0 / result;
        }

        result
    }
}

fn main() {
    println!("ans {:?}", Solution::my_pow(2.00000, -2147483648));
}
