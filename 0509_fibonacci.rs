impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let (mut n1, mut n2, mut curr) = (0, 1, 1);
        for _ in 1..n {
            curr = n1 + n2;
            n1 = n2;
            n2 = curr;
        }
        curr
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::fib(10));
}
