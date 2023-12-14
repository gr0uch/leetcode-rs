use std::env;

struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = height.len() - 1;
        let mut l_max: i32 = 0;
        let mut r_max: i32 = 0;
        let mut water_volume: i32 = 0;

        while l < r {
            l_max = l_max.max(height[l]);
            r_max = r_max.max(height[r]);
            if l_max < r_max {
                water_volume += l_max - height[l];
                l += 1;
            } else {
                water_volume += r_max - height[r];
                r -= 1;
            }
        }

        water_volume
    }
}

fn main() {
    let args: Vec<i32> = env::args().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("ans {}", Solution::trap(args));
}
