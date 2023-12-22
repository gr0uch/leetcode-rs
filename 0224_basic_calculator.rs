impl Solution {
    pub fn calculate(s: String) -> i32 {
        Self::calc(&mut s.chars())
    }

    fn calc<T: Iterator<Item = char>>(chars: &mut T) -> i32 {
        let mut result = 0;
        let mut sign = 1;
        let mut digits: Vec<u8> = vec![];

        while let Some(c) = chars.next() {
            if '0' <= c && c <= '9' {
                digits.push(c as u8);
                continue;
            } else if digits.len() > 0 {
                result += Self::cast_digits(&mut digits) * sign;
                sign = 1;
            }

            match c {
                '(' => {
                    result += Self::calc(chars) * sign;
                    sign = 1;
                }
                ')' => return result,
                '-' => sign = -sign,
                _ => (),
            }
        }

        result + Self::cast_digits(&mut digits) * sign
    }

    fn cast_digits(digits: &mut Vec<u8>) -> i32 {
        if digits.len() == 0 {
            return 0;
        }
        String::from_utf8(digits.drain(..).collect())
            .unwrap()
            .parse()
            .unwrap()
    }
}

struct Solution;

fn main() {
    println!(
        "ans {:?}",
        Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string())
    );
}
