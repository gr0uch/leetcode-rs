struct Solution {}

impl Solution {
    /// This one is pretty straightforward and more or less a direct
    /// interpretation of the prompt itself.
    /// time complexity O(mn)
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ones_row: Vec<i32> = vec![0; grid.len()];
        let mut ones_col: Vec<i32> = vec![0; grid[0].len()];
        let mut zeros_row: Vec<i32> = vec![0; grid.len()];
        let mut zeros_col: Vec<i32> = vec![0; grid[0].len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    0 => {
                        zeros_row[i] += 1;
                        zeros_col[j] += 1;
                    }
                    1 => {
                        ones_row[i] += 1;
                        ones_col[j] += 1;
                    }
                    _ => panic!("invalid input"),
                }
            }
        }

        let mut diff: Vec<Vec<i32>> = Vec::new();
        for i in 0..grid.len() {
            diff.push(Vec::new());
            for j in 0..grid[0].len() {
                diff[i].push(ones_row[i] + ones_col[j] - zeros_row[i] - zeros_col[j]);
            }
        }

        diff
    }
}

fn main() {
    println!(
        "ans {:?}",
        Solution::ones_minus_zeros(vec![vec![1, 1, 1], vec![1, 1, 1]])
    );
}
