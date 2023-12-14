use std::env;

struct Solution {}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Solution {
    /// This does two sweeps, one right and one left.
    /// On both sweeps, it tries to match peaks to a point that is equal or
    /// higher on the other side.
    /// Should be O(n).
    pub fn trap(height: Vec<i32>) -> i32 {
        Solution::trap_water(height)
    }

    fn trap_water(mut height: Vec<i32>) -> i32 {
        let mut water_volume: i32 = 0;

        water_volume += Solution::sweep(&mut height, Direction::Right);
        water_volume += Solution::sweep(&mut height, Direction::Left);

        water_volume
    }

    fn sweep(height: &mut Vec<i32>, direction: Direction) -> i32 {
        let mut water_volume: i32 = 0;
        let mut water_level: i32 = 0;
        let mut trailing_index: usize = match direction {
            Direction::Right => 0,
            Direction::Left => height.len() - 1,
        };

        for k in 0..height.len() {
            let i = match direction {
                Direction::Right => k,
                Direction::Left => height.len() - k - 1,
            };
            let column_height = height[i];
            if column_height >= water_level {
                let fill_length = i32::abs(i as i32 - trailing_index as i32) as usize;
                for m in 1..fill_length {
                    let j = match direction {
                        Direction::Right => i - m,
                        Direction::Left => i + m,
                    };
                    let h = height[j];
                    water_volume += water_level - h;
                    height[j] = water_level;
                }

                water_level = column_height;
                trailing_index = i;
            }
        }

        water_volume
    }
}

fn main() {
    let args: Vec<i32> = env::args().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("ans {}", Solution::trap(args));
}
