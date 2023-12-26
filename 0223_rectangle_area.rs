impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let mut result = ((ax2 - ax1) * (ay2 - ay1)).abs() + ((bx2 - bx1) * (by2 - by1)).abs();

        result -= (ax2.min(bx2) - ax1.max(bx1)).max(0) * (ay2.min(by2) - ay1.max(by1)).max(0);

        result
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
}
