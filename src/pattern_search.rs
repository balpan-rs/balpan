use aho_corasick::AhoCorasick;
#[derive(Debug, Clone)]
pub struct PatternTree;

type PatternPosition = (bool, Vec<usize>);

#[allow(clippy::new_without_default)]
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
