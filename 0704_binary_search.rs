use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lower = 0;
        let mut upper = nums.len();

        while lower < upper {
            let mid = (lower + upper) / 2;

            match target.cmp(&nums[mid]) {
                Ordering::Less => {
                    upper = mid;
                },
                Ordering::Greater => {
                    lower = mid + 1;
                },
                Ordering::Equal => {
                    return mid as i32;
                },
            }
        }

        -1
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::search(vec![2,5], 2));
}
