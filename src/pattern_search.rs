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

type PatternPosition = (bool, Vec<usize>);

impl PatternTree {
    pub fn new() -> Self {
        PatternTree
    }

    pub fn aho_corasick_search(&self, text: &str, patterns: &Vec<String>) -> PatternPosition {
        let ac = AhoCorasick::new(patterns).unwrap();
        let mut result: Vec<usize> = Vec::new();

        for mached in ac.find_iter(text) {
            result.push(mached.start());
        }

        (!result.is_empty(), result)
    }
}
