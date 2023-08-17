use aho_corasick::AhoCorasick;

//type Nodes = Rc<RefCell<TrieNode>>;

// #[derive(Debug, Clone, Default)]
// pub struct TrieNode {
//     children: BTreeMap<char, Nodes>,
//     failure_link: Option<Nodes>,
//     is_end_of_pattern: bool,
// }

// impl TrieNode {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// #[derive(Debug, Clone)]
// pub struct PatternTree {
//     root: Nodes,
// }

#[derive(Debug, Clone)]
pub struct PatternTree;

impl PatternTree {
    pub fn new() -> Self {
        PatternTree
    }

    pub fn aho_corasick_search(&self, text: &str, patterns: &Vec<String>) -> (bool, Vec<usize>) {
        let ac = AhoCorasick::new(patterns).unwrap();
        let mut result: Vec<usize> = Vec::new();

        for mached in ac.find_iter(text) {
            result.push(mached.start());
        }

        (!result.is_empty(), result)
    }

    // Stores where each character in the pattern appears last.
    // fn build_bad_character_table(&self, pattern: &str) -> HashMap<char, usize> {
    //     let mut table: HashMap<char, usize> = HashMap::new();
    //     let chars: Vec<char> = pattern.chars().collect();
    //     let pattern_len = pattern.len();
    //     let len = chars.len();

    //     for i in 0..len {
    //         table.insert(chars[i], len - i - 1);
    //     }

    //     table.insert(chars[len - 1], len);

    //     table
    // }

    // pub fn boyer_moore_search(&self, text: &str, pattern: &String) -> (bool, Vec<usize>) {
    //     if pattern.len() > text.len() {
    //         return (false, Vec::new());
    //     }

    //     let mut result: Vec<usize> = Vec::new();
    //     let bad_character_table = self.build_bad_character_table(pattern);
    //     let pattern_len = pattern.len();
    //     let text_len = text.len();
    //     let pattern_chars: Vec<char> = pattern.chars().collect();
    //     let text_chars: Vec<char> = text.chars().collect();
    //     let mut shift = 0; // Shift of the pattern with respect to the text

    //     while shift <= text_len - pattern_len {
    //         let mut j = pattern_len; // Start from pattern length

    //         // Keep reducing j while characters of pattern and text are matching at this shift
    //         while j > 0 && pattern_chars[j - 1] == text_chars[shift + j - 1] {
    //             j -= 1;
    //         }

    //         if j == 0 {
    //             result.push(shift);
    //             let next_shift = if shift + pattern_len < text_len {
    //                 pattern_len - bad_character_table.get(&text_chars[shift + pattern_len]).unwrap_or(&0)
    //             } else {
    //                 1
    //             };
    //
    //             shift += next_shift;
    //         } else {
    //             let char_shift = (j as isize - *bad_character_table.get(&text_chars[shift + j - 1]).unwrap_or(&0) as isize) as usize;
    //             shift += usize::max(1, char_shift);
    //         }
    //     }

    //     (result.len() > 0, result)
    // }
}
