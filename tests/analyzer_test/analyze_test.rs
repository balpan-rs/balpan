#[cfg(test)]
mod analyze_test {
    use indoc::indoc;
    use balpan::analyzer::{Analyzer, Traversable};
    use balpan::grammar::{fetch_grammars, build_grammars};
    use balpan::language::Language;

    fn assert_analyzed_source_code(source_code: &str, expected: &str, language: &str) {
        fetch_grammars().unwrap();
        build_grammars(None).unwrap();

        let analyzer = Analyzer {
            source_code: source_code.to_string(),
            language: Language::from(language),
        };

        let writer_queue = &analyzer.analyze();
        let mut string_vector = vec![];

        for line in writer_queue {
            string_vector.push(String::from(line));
        }


        let actual: String = string_vector
            // .iter()
            // .map( |str| { *str } )
            // .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_stacked_macros() {
        let source_code = indoc! {"
            #[derive(Deserialize)]
            #[serde(bound(deserialize = \"T: Deserialize<'de>\"))]
            struct List<T> {
                #[serde(deserialize_with = \"deserialize_vec\")]
                items: Vec<T>,
            }"};

        let result = indoc! {"
            /// [TODO]
            #[derive(Deserialize)]
            #[serde(bound(deserialize = \"T: Deserialize<'de>\"))]
            struct List<T> {
                #[serde(deserialize_with = \"deserialize_vec\")]
                items: Vec<T>,
            }"};

        assert_analyzed_source_code(source_code, result, "rust")
    }

    #[test]
    fn test_ignore_todo_test_macro() {
        let source_code = indoc! {"
            #[cfg(test)]
            mod tests {
                use super::*;

                #[test]
                fn test_foo() {
                    assert_eq!(foo(), 1);
                }
            }"};

        let result = indoc! {"
            /// [TODO]
            #[cfg(test)]
            mod tests {
                use super::*;

                /// [TODO]
                #[test]
                fn test_foo() {
                    assert_eq!(foo(), 1);
                }
            }"};

        assert_analyzed_source_code(source_code, result, "rust")
    }

    #[test]
    fn test_ignore_doc_macro() {
        let source_code = indoc! {"
            #[doc = \"This is a doc comment\"]
            fn foo() {
                println!(\"foo\");
            }"};

        let result = indoc! {"
            /// [TODO]
            #[doc = \"This is a doc comment\"]
            fn foo() {
                println!(\"foo\");
            }"};

        assert_analyzed_source_code(source_code, result, "rust")
    }

    #[test]
    fn test_trait_and_impl() {
        let source_code = indoc! { "
        pub trait RangeFactory {
            fn from_node(node: Node) -> Range;
        }

        impl RangeFactory for Range {
            #[inline]
            fn from_node(node: Node) -> Range {
                Range {
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    start_point: node.start_position(),
                    end_point: node.end_position(),
                }
            }
        }"};

        let result = indoc! { "
        /// [TODO]
        pub trait RangeFactory {
            fn from_node(node: Node) -> Range;
        }

        /// [TODO]
        impl RangeFactory for Range {
            /// [TODO]
            #[inline]
            fn from_node(node: Node) -> Range {
                Range {
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    start_point: node.start_position(),
                    end_point: node.end_position(),
                }
            }
        }"};

        assert_analyzed_source_code(source_code, result, "rust")
    }

    #[test]
    fn test_trait_and_impl_with_mod() {
        let source_code = indoc! { "
        mod tree_sitter_extended {
            pub trait RangeFactory {
                fn from_node(node: Node) -> Range;
            }

            impl RangeFactory for Range {
                #[inline]
                fn from_node(node: Node) -> Range {
                    Range {
                        start_byte: node.start_byte(),
                        end_byte: node.end_byte(),
                        start_point: node.start_position(),
                        end_point: node.end_position(),
                    }
                }
            }
        }"};

        let result = indoc! { "
        /// [TODO]
        mod tree_sitter_extended {
            /// [TODO]
            pub trait RangeFactory {
                fn from_node(node: Node) -> Range;
            }

            /// [TODO]
            impl RangeFactory for Range {
                /// [TODO]
                #[inline]
                fn from_node(node: Node) -> Range {
                    Range {
                        start_byte: node.start_byte(),
                        end_byte: node.end_byte(),
                        start_point: node.start_position(),
                        end_point: node.end_position(),
                    }
                }
            }
        }"};

        assert_analyzed_source_code(source_code, result, "rust")
    }
}
