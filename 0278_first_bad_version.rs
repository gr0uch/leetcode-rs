// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lower = 1;
        let mut upper = n;
        while lower != upper {
            let mid = ((lower as i64 + upper as i64) / 2) as i32;
            let is_bad = self.isBadVersion(mid);
            match is_bad {
                false => {
                    lower = mid + 1;
                },
                true => {
                    upper = mid;
                },
            }
        }
        lower
    }

    fn isBadVersion(&self, n: i32) -> bool {
        n >= 1702766719
    }
}

struct Solution;

fn main() {
    let soln = Solution {};
    println!("ans {:?}", soln.first_bad_version(2126753390));
}
