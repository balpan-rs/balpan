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