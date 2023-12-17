use std::collections::BTreeMap;

#[derive(Default)]
struct Trie {
    next: BTreeMap<u8, Trie>,
    word: Option<String>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    pub fn add_word(&mut self, word: String) {
        let mut cursor = self;
        let bytes = word.as_bytes();
        for i in 0..word.len() {
            let c = bytes[i];

            cursor = cursor.next.entry(c).or_insert(Trie::new());
        }

        cursor.word = Some(word);
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        Self::do_find(board, words)
    }

    fn do_find(board: Vec<Vec<char>>, mut words: Vec<String>) -> Vec<String> {
        let mut root = Trie::new();
        let mut remainder = words.len();

        words.drain(..).for_each(|word| {
            root.add_word(word)
        });

        let height = board.len();
        let width = board[0].len();
        let mut bboard = board;
        for y in 0..height {
            for x in 0..width {
                Self::dfs(&mut bboard, x, y, &mut root, &mut remainder, &mut words);
            }
        }

        words
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
        trie: &mut Trie,
        remainder: &mut usize,
        result: &mut Vec<String>,
    ) {
        let c = board[y][x] as u8;
        if c == 0 {
            return;
        }
        let next_node = match trie.next.get_mut(&c) {
            None => return,
            Some(node) => node,
        };
        if let Some(word) = next_node.word.take() {
            result.push(word);
            *remainder -= 1;
            if *remainder == 0 {
                return;
            }
        }

        board[y][x] = 0 as char;
        if x > 0 {
            Self::dfs(board, x - 1, y, next_node, remainder, result);
        }
        if y > 0 {
            Self::dfs(board, x, y - 1, next_node, remainder, result);
        }
        if x < board[0].len() - 1 {
            Self::dfs(board, x + 1, y, next_node, remainder, result);
        }
        if y < board.len() - 1 {
            Self::dfs(board, x, y + 1, next_node, remainder, result);
        }
        board[y][x] = c as char;
    }
}

struct Solution;

fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['a', 'b', 'c'],
        vec!['a', 'e', 'd'],
        vec!['a', 'f', 'g'],
    ];
    let words: Vec<String> = vec![
        "abcdefg".to_string(),
        "gfedcbaaa".to_string(),
        "eaabcdgfa".to_string(),
        "befa".to_string(),
        "dgc".to_string(),
        "ade".to_string(),
    ];
    println!("ans {:?}", Solution::find_words(board, words));
}
