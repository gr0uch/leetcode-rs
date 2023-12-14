use std::env;

struct Solution {}

impl Solution {
    /// this is a pretty bad O(n^2) solution, there is an outer loop that
    /// iterates on nums and an inner loop that iterates on nums after the
    /// current iteration.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i: usize = 0;
        let mut j: usize = 1;
        let len = nums.len();

        while i < len - 1 {
            while j < len {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
                j += 1;
            }

            i += 1;
            j = i + 1;
        }

        vec![]
    }
}

fn main() {
    let mut args: Vec<i32> = env::args().filter_map(|s| s.parse::<i32>().ok()).collect();
    let target = args.pop().unwrap();
    println!("ans {:?}", Solution::two_sum(args, target));
}
