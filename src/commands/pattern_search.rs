use crate::commands::boyer_moore::{BoyerMooreSearch, SearchIn};
use aho_corasick::AhoCorasick;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct PatternTree {
    pub ignore_case: bool,
    pub regex_flag: bool,
}

type PatternPosition = (bool, Vec<usize>);

#[allow(clippy::new_without_default)]
impl PatternTree {
    pub fn new() -> Self {
        PatternTree {
            ignore_case: false,
            regex_flag: false,
        }
    }

    /// Call all search methods based on the given patterns
    ///
    /// If the pattern is single, then call `boyer_moore_search` method.
    /// Because BM algorithm is known as the fastest algorithm for single pattern search.
    ///
    /// Whereas, if the pattern is multiple, then call `aho_corasick_search` method.
    /// AC is known as the fastest algorithm for multiple pattern search.
    pub fn selective_search(&self, patterns: &Vec<String>, text: &str) -> PatternPosition {
        if self.regex_flag {
            return self.regex(text, &patterns[0]);
        }

        match patterns.len() {
            0 => (false, vec![]),
            1 => match self.ignore_case {
                true => self.boyer_moore_search(&text.to_lowercase(), &patterns[0].to_lowercase()),
                false => self.boyer_moore_search(text, &patterns[0]),
            },
            _ => {
                if self.ignore_case {
                    let mut lower_patterns: Vec<String> = Vec::new();
                    patterns
                        .iter()
                        .for_each(|pattern| lower_patterns.push(pattern.to_lowercase()));
                    self.aho_corasick_search(&text.to_lowercase(), &lower_patterns)
                } else {
                    self.aho_corasick_search(text, patterns)
                }
            }
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

    pub fn regex(&self, text: &str, pattern: &String) -> PatternPosition {
        let re = match self.ignore_case {
            true => Regex::new(&format!(r"(?i){}", pattern)).unwrap(),
            false => Regex::new(pattern).unwrap(),
        };

        let mut result: Vec<usize> = Vec::new();

        for matched in re.find_iter(text) {
            result.push(matched.start());
        }

        (!result.is_empty(), result)
    }
}
