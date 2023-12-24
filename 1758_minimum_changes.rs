impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let (mut opt_a, mut opt_b) = (0, 0);
        let mut expected = '0';

        for c in s.chars() {
            if c == expected {
                opt_b += 1;
            } else {
                opt_a += 1;
            }

            expected = if expected == '0' { '1' } else { '0' };
        }

        opt_a.min(opt_b)
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::min_operations("0100".to_string()));
}
