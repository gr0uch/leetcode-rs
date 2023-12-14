use std::env;

struct Solution {}

impl Solution {
    /// I think solution is a bit unusual. It "follows" indices and tries to
    /// set the corresponding value for that index.
    /// For example, [2, 3, 1] will be rewritten to [1, 2, 3].
    /// If any value is not the expected value, it returns.
    /// This should be an O(n) soln.
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        Solution::fmp(nums)
    }

    fn fmp(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            Solution::eraser(&mut nums, i);
        }

        for i in 0..nums.len() {
            let value = *nums.get(i as usize).unwrap();
            let expected_value = (i + 1) as i32;
            if value != expected_value {
                return expected_value;
            }
        }

        return (nums.len() + 1) as i32;
    }

    fn eraser(nums: &mut Vec<i32>, mut index: usize) {
        let mut is_initial = true;
        loop {
            let value = *nums.get(index).unwrap();
            if !is_initial {
                nums[index] = (index + 1) as i32;
            }
            if value < 1 {
                break;
            }
            let target_index = (value - 1) as usize;
            if target_index > nums.len() - 1 {
                break;
            }
            if index == target_index {
                break;
            }
            is_initial = false;
            index = target_index;
        }
    }
}

fn main() {
    let args: Vec<i32> = env::args().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("ans {}", Solution::first_missing_positive(args));
}
