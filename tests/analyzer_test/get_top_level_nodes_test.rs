#[cfg(test)]
mod get_top_level_nodes_test {
    use balpan::analyzer::{Analyzer,Traversable};
    use balpan::grammar::{fetch_grammars, build_grammars};

    fn assert_top_level_node_kinds(source_code: &str, expected: Vec<&str>) {
        fetch_grammars().unwrap();
        build_grammars(None).unwrap();

        let analyzer = Analyzer { 
            source_code: source_code.to_string() 
        };

        let tree = analyzer.get_syntax_tree();
        let nodes = analyzer.get_top_level_nodes(&tree);

        let result: Vec<&str> = nodes
            .iter()
            .map( |node| { node.kind() } )
            .collect();

        assert!(
            expected.iter().eq(
                result.iter()
            )
        );
    }

    #[test]
    fn test_declaring_error_enum_with_macro() {
        let source_code = r#"
        use thiserror::Error;

        #[derive(Error, Debug)]
        pub enum FormatError {
            #[error("Invalid header (expected {expected:?}, got {found:?})")]
            InvalidHeader {
                expected: String,
                found: String,
            },
            #[error("Missing attribute: {0}")]
            MissingAttribute(String),
        }
        "#;

        let expected = vec!["use_declaration", "attribute_item", "enum_item"];

        assert_top_level_node_kinds(source_code, expected);
    }

    #[test]
    fn test_macro_invocation_statement() {
        let source_code = r#"
        use target_spec::eval;

        // Evaluate Rust-like `#[cfg]` syntax.
        let cfg_target = "cfg(all(unix, target_arch = \"x86_64\"))";
        assert_eq!(eval(cfg_target, "x86_64-unknown-linux-gnu").unwrap(), Some(true));
        assert_eq!(eval(cfg_target, "i686-unknown-linux-gnu").unwrap(), Some(false));
        assert_eq!(eval(cfg_target, "x86_64-pc-windows-msvc").unwrap(), Some(false));

        // Evaluate a full target-triple.
        assert_eq!(eval("x86_64-unknown-linux-gnu", "x86_64-unknown-linux-gnu").unwrap(), Some(true));
        assert_eq!(eval("x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc").unwrap(), Some(false));    
        "#;

        let expected = vec![
            "use_declaration",
            "line_comment",
            "let_declaration",
            "expression_statement",
            "expression_statement",
            "expression_statement",
            "line_comment",
            "expression_statement",
            "expression_statement",
        ];

        assert_top_level_node_kinds(source_code, expected)
    }
}
