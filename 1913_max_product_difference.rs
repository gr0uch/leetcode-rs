impl Solution {
    /// just find the highest 2 and lowest 2
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        // high 1, high 2, low 1, low 2
        let mut counter: (i32, i32, i32, i32) = (0, 0, i32::MAX, i32::MAX);

        for n in nums {
            if n > counter.1 {
                counter.1 = n;
                if counter.1 > counter.0 {
                    counter.1 = counter.0;
                    counter.0 = n;
                }
            }
            if n < counter.3 {
                counter.3 = n;
                if counter.3 < counter.2 {
                    counter.3 = counter.2;
                    counter.2 = n;
                }
            }
        }

        counter.0 * counter.1 - counter.2 * counter.3
    }
}

struct Solution;

fn main() {
    println!(
        "ans {:?}",
        Solution::max_product_difference(vec![5, 6, 2, 7, 4])
    );
}
