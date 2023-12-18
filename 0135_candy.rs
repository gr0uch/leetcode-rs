impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut result: i32 = 1;
        let mut counter: (i32, i32, i32) = (0, 0, 0);
        for i in 0..ratings.len() - 1 {
            let r_i = ratings[i];
            let r_j = ratings[i + 1];
            if r_i < r_j {
                counter = (counter.1 + 1, counter.1 + 1, 0);
                result += counter.1 + 1;
            } else if r_i == r_j {
                counter = (0, 0, 0);
                result += 1;
            } else {
                counter = (counter.0, 0, counter.2 + 1);
                result += counter.2 + 1;
                if counter.0 >= counter.2 {
                    result -= 1;
                }
            }
        }
        result
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::candy(vec![1,0,2]));
}
