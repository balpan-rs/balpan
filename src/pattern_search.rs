use std::{collections::{BTreeMap, VecDeque, HashMap}, rc::Rc, cell::RefCell};

type Nodes = Rc<RefCell<TrieNode>>;

#[derive(Debug, Clone, Default)]
pub struct TrieNode {
    children: BTreeMap<char, Nodes>,
    failure_link: Option<Nodes>,
    is_end_of_pattern: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct PatternTree {
    root: Nodes,
}

impl PatternTree {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(TrieNode::new())),
        }
    }

    /// Use an adaptive approach that selects the appropriate algorithm 
    /// based on the number or shape of patterns to be searched. 
    /// 
    /// applied strategies using Boyer-Moore for single patterns 
    /// and Commentz-Walter for multiple patterns.
    pub fn adaptive_search(&mut self, text: &str, patterns: &[String]) -> Vec<usize> {
        if patterns.len() == 1 {
            // apply boyer-moore algorithm for single pattern search
            return self.boyer_moore_search(text, &patterns[0]);
        }

        // for multi pattern search
        self.add_patterns(patterns);
        self.build_failure_links();
        self.search_pattern(text)
    }

    pub fn add_patterns(&mut self, patterns: &[String]) {
        patterns.iter().for_each(|pat| self.add_single_pattern(pat))
    }

    fn add_single_pattern(&mut self, pattern: &str) {
        let mut current_node = self.root.clone();

        for ch in pattern.chars() {
            let child_node = current_node
                .borrow()
                .children
                .get(&ch)
                .cloned()
                .unwrap_or_else(|| {
                    let new_node = Rc::new(RefCell::new(TrieNode::new()));
                    current_node.borrow_mut().children.insert(ch, new_node.clone());
                    new_node
                });
            current_node = child_node;
        }

        current_node.borrow_mut().is_end_of_pattern = true;
    }

    fn search_pattern(&self, text: &str) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let mut current_node = self.root.clone();

        for (index, ch) in text.chars().enumerate() {
            loop {
                let next_node: Option<Nodes> = current_node.borrow().children.get(&ch).cloned()
                    .or_else(|| current_node.borrow().failure_link.clone());
    
                match next_node {
                    Some(node) => current_node = node,
                    None => break,
                }
            }
    
            if current_node.borrow().is_end_of_pattern {
                result.push(index);
            }
        }

        result
    }

    pub fn build_failure_links(&self) {
        let mut queue: VecDeque<Nodes> = VecDeque::new();
        let root = self.root.clone();

        // Add the child nodes of the root node to the queue and set the failure link as root
        for child in root.borrow().children.values() {
            queue.push_back(child.clone());
            child.borrow_mut().failure_link = Some(root.clone());
        }

        while let Some(node) = queue.pop_front() {
            for (ch, child) in node.borrow().children.iter() {
                let mut failure_link = node.borrow().failure_link.clone();

                while let Some(failure_node) = failure_link {
                    if failure_node.borrow().children.contains_key(ch) {
                        child.borrow_mut().failure_link = failure_node.borrow().children.get(ch).cloned();
                        break;
                    }
                    
                    failure_link = failure_node.borrow().failure_link.clone();
                }

                if child.borrow().failure_link.is_none() {
                    child.borrow_mut().failure_link = Some(root.clone());
                }

                queue.push_back(child.clone());
            }
        }
    }

    /// Stores where each character in the pattern appears last.
    fn build_bad_character_table(&self, pattern: &str) -> HashMap<char, usize> {
        let mut table: HashMap<char, usize> = HashMap::new();
        let chars: Vec<char> = pattern.chars().collect();
        // let pattern_len = pattern.len();
        let len = chars.len();
    
        for i in 0..len {
            table.insert(chars[i], len - i - 1);
        }

        table
    }

    fn boyer_moore_search(&self, text: &str, pattern: &str) -> Vec<usize> {
        if pattern.len() > text.len() {
            return Vec::new();
        }
    
        let mut result: Vec<usize> = Vec::new();
        let bad_character_table = self.build_bad_character_table(pattern);
        let pattern_len = pattern.len();
        let text_len = text.len();
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let text_chars: Vec<char> = text.chars().collect();
        let mut shift = 0; // Shift of the pattern with respect to the text
    
        while shift <= text_len - pattern_len {
            let mut j = pattern_len; // Start from pattern length
    
            // Keep reducing j while characters of pattern and text are matching at this shift
            while j > 0 && pattern_chars[j - 1] == text_chars[shift + j - 1] {
                j -= 1;
            }
    
            if j == 0 {
                result.push(shift);
                let next_shift = if shift + pattern_len < text_len {
                    pattern_len - bad_character_table.get(&text_chars[shift + pattern_len]).unwrap_or(&0)
                } else {
                    1
                };
                shift += next_shift;
            } else {
                let char_shift = (j as isize - *bad_character_table.get(&text_chars[shift + j - 1]).unwrap_or(&0) as isize) as usize;
                shift += usize::max(1, char_shift);
            }
        }

        println!("shift: {}", shift);

        result
    }
}

#[cfg(test)]
mod pattern_search_tests {
    #[test]
    fn test_boyer_moore_search() {
        use super::PatternTree;

        let searcher = PatternTree::new();

        let text = "ABAAABCDABCDABABCD";
        let pattern = "ABCD";
        let expected = vec![4, 8, 14];
        assert_eq!(searcher.boyer_moore_search(text, pattern), expected);

        let pattern_not_found = "XYZ";
        let expected_empty = Vec::<usize>::new();
        assert_eq!(searcher.boyer_moore_search(text, pattern_not_found), expected_empty);
    }

    #[test]
    fn test_boyer_moore_search_contains_new_line() {
        use super::PatternTree;

        let searcher = PatternTree::new();

        let text = "ABAA\nABCDABCD\nABABCD\n";
        let pattern = "ABCD";
        let expected = vec![5, 9, 16];
        assert_eq!(searcher.boyer_moore_search(text, pattern), expected);
    }

    #[test]
    fn test_boyer_moore_search_slash_character() {
        use super::PatternTree;

        let searcher = PatternTree::new();
        let text = r#"
        /// [TODO] aaa
        /// some comment
        struct AAA {
            field: i32,
        };

        /// [TODO] bbb
        /// some comment about bbb
        fn bbb() {
            unimplemented!();
        }"#;
        let pattern = "[TODO]";

        for (i, line) in text.lines().enumerate() {
            searcher.boyer_moore_search(line, pattern);

            match i {
                1 => {
                    let expected = vec![12];
                    assert_eq!(searcher.boyer_moore_search(line, pattern), expected);
                },
                7 => {
                    let expected = vec![12];
                    assert_eq!(searcher.boyer_moore_search(line, pattern), expected);
                }
                _ => {
                    let expected = Vec::<usize>::new();
                    assert_eq!(searcher.boyer_moore_search(line, pattern), expected);
                }
            }
        }
    }
}
