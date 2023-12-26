impl Solution {
    /// this solution is not g8...
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        for y in 0..matrix.len() {
            for x in 0..matrix[0].len() {
                let area = Self::check_char(&matrix, y, x);
                if area > result {
                    result = area;
                }
            }
        }
        result
    }

    fn check_char(matrix: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
        let starting_char = matrix[y][x];
        if starting_char != '1' { return 0; }

        let mut h = 0;
        let mut histogram = vec![];

        while y + h < matrix.len() {
            if matrix[y + h][x] != '1' { break; }
            let max_w = if histogram.len() > 0 { histogram[h - 1] } else { matrix[0].len() - x };
            for w in 0..=max_w {
                if w == max_w || matrix[y + h][x + w] != '1' {
                    histogram.push(w);
                    break;
                }
            }
            h += 1;
        }

        let mut max = 1;

        for row in 0..histogram.len() {
            let area = (row + 1) * histogram[row];
            if area > max { max = area; }
        }

        max as i32
    }
}

struct Solution;

fn main() {
    println!("ans {:?}", Solution::maximal_rectangle(
        vec![
            vec!['1','1','1','1','1','1','1','1'],
            vec!['1','1','1','1','1','1','1','0'],
            vec!['1','1','1','1','1','1','1','0'],
            vec!['1','1','1','1','1','0','0','0'],
            vec!['0','1','1','1','1','0','0','0']
        ]
    ));
    println!("ans {:?}", Solution::maximal_rectangle(
        vec![
            vec!['1','0','1','0','0'],
            vec!['1','0','1','1','1'],
            vec!['1','1','1','1','1'],
            vec!['1','0','0','1','0']
        ]
    ));
}
