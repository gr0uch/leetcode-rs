impl Solution {
    /// merge sort
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut partitions: Vec<_> = nums.iter().map(|n| vec![*n]).collect();

        while partitions.len() > 1 {
            let mut new_partitions = vec![];

            for i in (0..partitions.len()).step_by(2) {
                let curr = &partitions[i];
                if partitions.get(i + 1).is_none() {
                    new_partitions.push(curr.to_vec());
                    break;
                }

                let mut part = vec![];
                let next = &partitions[i + 1];

                let mut i_curr: usize = 0;
                let mut i_next: usize = 0;

                for _ in 0..(curr.len() + next.len()) {
                    let has_next = i_next < next.len();
                    let has_curr = i_curr < curr.len();
                    if !has_next || (has_next && has_curr && curr[i_curr] < next[i_next]) {
                        part.push(curr[i_curr]);
                        i_curr += 1;
                    } else {
                        part.push(next[i_next]);
                        i_next += 1;
                    }
                }

                new_partitions.push(part);
            }

            partitions = new_partitions;
        }

        partitions[0].to_owned()
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::sort_array(vec![5, 1, 1, 2, 0, 0]));
}
