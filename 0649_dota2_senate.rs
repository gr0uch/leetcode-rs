use std::env;
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    /// Uses two queues, popping and pushing until a winner is found.
    /// O(n)
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant_senators: VecDeque<usize> = VecDeque::new();
        let mut dire_senators: VecDeque<usize> = VecDeque::new();

        for (i, c) in senate.char_indices() {
            match c {
                'R' => radiant_senators.push_back(i),
                'D' => dire_senators.push_back(i),
                _ => panic!("faulty input"),
            }
        }

        let mut running_index = senate.len();
        while radiant_senators.len() > 0 && dire_senators.len() > 0 {
            if radiant_senators[0] < dire_senators[0] {
                radiant_senators.push_back(running_index);
            } else {
                dire_senators.push_back(running_index);
            }
            radiant_senators.pop_front();
            dire_senators.pop_front();
            running_index += 1;
        }

        if radiant_senators.len() > 0 {
            "Radiant".to_string()
        } else {
            "Dire".to_string()
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let senate = args.pop().unwrap();
    println!("ans {:?}", Solution::predict_party_victory(senate));
}
