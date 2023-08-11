
#[cfg(test)]
mod analyze_test {
    use indoc::indoc;
    use balpan::analyzer::{Analyzer,Traversable};
    use balpan::grammar::{fetch_grammars, build_grammars};

    fn assert_analyzed_source_code(source_code: &str, expected: &str, language: &str) {
        fetch_grammars().unwrap();
        build_grammars(None).unwrap();

        let analyzer = Analyzer {
            source_code: source_code.to_string(),
            language: language.to_string(),
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

    #[test]
    fn test_python_function() {
        let source_code = indoc! { "
        def foo():
            print('foo')"};

        let expected = indoc! { "
        # [TODO]
        def foo():
            print('foo')"};

        assert_analyzed_source_code(source_code, expected, "python");

        // python closure
        let source_code = indoc! { "
        def outer_function():
            def inner_function():
                print('foo')
            return inner_function"};

        let expected = indoc! { "
        # [TODO]
        def outer_function():
            # [TODO]
            def inner_function():
                print('foo')
            return inner_function"};

        assert_analyzed_source_code(source_code, expected, "python")
    }

    #[test]
    fn test_python_class_and_function() {
        let source_code = indoc! { "
        class Foo:
            def __init__(self, value = 0):
                self.value = value

            def add(self, value: int):
                self.value += value" };

        let expected = indoc! { "
        # [TODO]
        class Foo:
            # [TODO]
            def __init__(self, value = 0):
                self.value = value

            # [TODO]
            def add(self, value: int):
                self.value += value" };

        assert_analyzed_source_code(source_code, expected, "python")
    }

    #[test]
    fn test_python_contain_decorator() {
        let source_code = indoc! { "
        def debug_func(func):
            def wrapper(*args, **kwargs):
                print(f\"Calling {func.__name__} with arguments {args} and keyword arguments {kwargs}\")
                return func(*args, **kwargs)
            return wrapper
        
        @debug_func
        def foo(a: int, b: int) -> int:
            return a + b" };

        let expected = indoc! { "
        # [TODO]
        def debug_func(func):
            # [TODO]
            def wrapper(*args, **kwargs):
                print(f\"Calling {func.__name__} with arguments {args} and keyword arguments {kwargs}\")
                return func(*args, **kwargs)
            return wrapper

        # [TODO]
        @debug_func
        def foo(a: int, b: int) -> int:
            return a + b" };

        assert_analyzed_source_code(source_code, expected, "python")
    }

    #[test]
    fn test_python_lambda_and_hof_no_comments() {
        let source_code = indoc! { "
        multiply = lambda x, y: x * y

        numbers = [1, 2, 3, 4]
        doubled = map(lambda x: x * 2, numbers)" };

        let expected = indoc! { "
        multiply = lambda x, y: x * y

        numbers = [1, 2, 3, 4]
        doubled = map(lambda x: x * 2, numbers)" };

        assert_analyzed_source_code(source_code, expected, "python")
    }
}
