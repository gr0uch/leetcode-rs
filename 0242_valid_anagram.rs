impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let count_s: &mut [u32] = &mut [0; 26];
        let count_t: &mut [u32] = &mut [0; 26];
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        for i in 0..s.len() {
            let c_s = sb[i];
            let c_t = tb[i];
            count_s[(c_s - b'a') as usize] += 1;
            count_t[(c_t - b'a') as usize] += 1;
        }

        for i in 0..26 {
            if count_s[i] != count_t[i] {
                return false;
            }
        }

        true
    }
}

struct Solution;

fn main() {
    println!(
        "ans {:?}",
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string())
    );
}
