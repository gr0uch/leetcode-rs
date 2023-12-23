use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let mut max = 0;

        for i_upper in 0..s.len() {
            let c_upper = s.as_bytes()[i_upper];
            let has_char = !set.insert(c_upper);
            if has_char {
                let mut i_lower = i_upper - set.len();
                while i_lower < i_upper {
                    let c_lower = s.as_bytes()[i_lower];
                    if c_lower == c_upper {
                        break;
                    }
                    set.remove(&c_lower);
                    i_lower += 1;
                }
            } else if set.len() > max {
                max = set.len();
            }
        }

        max as i32
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::length_of_longest_substring("ohvhvhjdml".to_string()));
}
