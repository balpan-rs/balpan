#[cfg(test)]
mod pattern_search_test {
    #[test]
    fn adaptive_search_multiple_patterns_test() {
        use balpan::pattern_search::PatternTree;

        let mut searcher = PatternTree::new();
        let patterns = vec!["[TODO]".to_string(), "[DONE]".to_string()];

        let text = r#"
        /// [TODO] ABC
        /// [DONE] some comment"#;
        
        let expected = (true, vec![13, 36]);

        let result = searcher.adaptive_search(text, &patterns);

        assert_eq!(result, expected);
    }

    #[test]
    fn adaptive_search_single_pattern_test() {
        use balpan::pattern_search::PatternTree;

        let mut searcher = PatternTree::new();
        let patterns = vec!["[TODO]".to_string()];

        let text = r#"
        /// [TODO] ABC
        /// [DONE] some comment"#;
        
        let expected = (true, vec![12]);

        for (i, line) in text.lines().enumerate() {
            let result = searcher.adaptive_search(line, &patterns);

            match i {
                1 => assert_eq!(result, expected),
                _ => assert_eq!(result, (false, vec![])),
            }
        }
    }
}

#[cfg(test)]
mod boyer_moore_search_test {
    #[test]
    fn test_boyer_moore_search() {
        use balpan::pattern_search::PatternTree;

        let searcher = PatternTree::new();

        let text = "ABAAABCDABCDABABCD";
        let pattern = "ABCD".to_string();
        let expected = (true, vec![4, 8, 14]);
        assert_eq!(searcher.boyer_moore_search(text, &pattern), expected);

        let pattern_not_found = "XYZ".to_string();
        let expected_empty = (false, vec![]);
        assert_eq!(searcher.boyer_moore_search(text, &pattern_not_found), expected_empty);
    }

    #[test]
    fn test_boyer_moore_search_contains_new_line() {
        use balpan::pattern_search::PatternTree;

        let searcher = PatternTree::new();

        let text = "ABAA\nABCDABCD\nABABCD\n";
        let pattern = "ABCD".to_string();
        let expected = (true, vec![5, 9, 16]);
        assert_eq!(searcher.boyer_moore_search(text, &pattern), expected);
    }

    #[test]
    fn test_boyer_moore_search_slash_character() {
        use balpan::pattern_search::PatternTree;

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
        let pattern = "[TODO]".to_string();

        for (i, line) in text.lines().enumerate() {
            searcher.boyer_moore_search(line, &pattern);

            match i {
                1 => {
                    let expected = (true, vec![12]);
                    assert_eq!(searcher.boyer_moore_search(line, &pattern), expected);
                },
                7 => {
                    let expected = (true, vec![12]);
                    assert_eq!(searcher.boyer_moore_search(line, &pattern), expected);
                }
                _ => {
                    let expected = (false, vec![]);
                    assert_eq!(searcher.boyer_moore_search(line, &pattern), expected);
                }
            }
        }
    }
}

#[cfg(test)]
mod aho_corasick_search_test {
    #[test]
    fn test_search_multiple_pattern() {
        use balpan::pattern_search::PatternTree;

        let searcher = PatternTree::new();

        let text = "ABAAABCDABCDABABCD";
        let patterns = vec!["ABCD".to_string(), "BCD".to_string()];

        let expected = (true, vec![4, 8, 14]);
        assert_eq!(searcher.aho_corasick_search(text, &patterns), expected);
    }

    #[test]
    fn test_search_todo_done_comments_using_aho_corasick() {
        use balpan::pattern_search::PatternTree;

        let searcher = PatternTree::new();
        let text = r#"
        /// [TODO] ABC
        /// some comment
        /// struct ABC {
        ///     field: i32,
        ///     field2: i32,
        /// }
        /// 
        /// [TODO] DEF
        /// some comment about DEF
        /// fn DEF() {
        ///    unimplemented!();
        /// }
        /// "#;
        
        let patterns = vec!["[TODO]".to_string(), "[DONE]".to_string()];
        let expected = (true, vec![13, 170]);

        let result = searcher.aho_corasick_search(text, &patterns);

        assert_eq!(result, expected);
    }
}