impl Solution {
    /// sliding window where window length is the sum of all 1s
    /// we care about contiguous 1s at the end
    /// we are basically keeping track of how many 1s in the current window
    /// then diffing the sum from the max # of 1s in any window
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let len = nums.len();
        let mut swaps = i32::MAX;
        let mut curr = 0;
        let mut i_lower = 0;
        for i_upper in 0..(len * 2) {
            curr += nums[i_upper % len];
            if i_upper >= sum as usize {
                curr -= nums[i_lower % len];
                i_lower += 1;
            }
            swaps = swaps.min(sum - curr);
        }

        swaps
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]));
}
