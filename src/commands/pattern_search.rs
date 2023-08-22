use aho_corasick::AhoCorasick;
use crate::commands::boyer_moore::{BoyerMooreSearch, SearchIn};

#[derive(Debug, Clone)]
pub struct PatternTree;

type PatternPosition = (bool, Vec<usize>);

#[allow(clippy::new_without_default)]
impl PatternTree {
    pub fn new() -> Self {
        PatternTree
    }

    /// Call all search methods based on the given patterns
    /// 
    /// If the pattern is single, then call `boyer_moore_search` method. 
    /// Because BM algorithm is known as the fastest algorithm for single pattern search.
    /// 
    /// Whereas, if the pattern is multiple, then call `aho_corasick_search` method.
    /// AC is known as the fastest algorithm for multiple pattern search.
    pub fn selective_search(&self, patterns: &Vec<String>, text: &str) -> PatternPosition {
        match patterns.len() {
            0 => (false, vec![]),
            1 => self.boyer_moore_search(text, &patterns[0]),
            _ => self.aho_corasick_search(text, patterns),
        }
    }

    pub fn aho_corasick_search(&self, text: &str, patterns: &Vec<String>) -> PatternPosition {
        let ac = AhoCorasick::new(patterns).unwrap();
        let mut result: Vec<usize> = Vec::new();

        for matched in ac.find_iter(text) {
            result.push(matched.start());
        }

        (!result.is_empty(), result)
    }

    pub fn boyer_moore_search(&self, text: &str, pattern: &String) -> PatternPosition {
        let searcher = BoyerMooreSearch::new(pattern.as_bytes());
        let result: Vec<usize> = searcher.find_in(text.as_bytes()).collect();

        (!result.is_empty(), result)
    }
}
