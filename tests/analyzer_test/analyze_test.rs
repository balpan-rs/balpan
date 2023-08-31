use balpan::analyzer::Analyzer;
use balpan::grammar::{build_grammars, fetch_grammars};
use balpan::language::Language;
use indoc::indoc;

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
    let source_code = indoc! {r#"
        #[derive(Deserialize)]
        #[serde(bound(deserialize = "T: Deserialize<'de>"))]
        struct List<T> {
            #[serde(deserialize_with = "deserialize_vec")]
            items: Vec<T>,
        }"#};

    let result = indoc! {r#"
        /// [TODO] List
        #[derive(Deserialize)]
        #[serde(bound(deserialize = "T: Deserialize<'de>"))]
        struct List<T> {
            #[serde(deserialize_with = "deserialize_vec")]
            items: Vec<T>,
        }"#};

    assert_analyzed_source_code(source_code, result, "rust")
}

#[test]
fn test_idempotency() {
    let source_code = indoc! {r#"
        /// [TODO] List
        #[derive(Deserialize)]
        #[serde(bound(deserialize = "T: Deserialize<'de>"))]
        struct List<T> {
            #[serde(deserialize_with = "deserialize_vec")]
            items: Vec<T>,
        }"#};

    let result = indoc! {r#"
        /// [TODO] List
        #[derive(Deserialize)]
        #[serde(bound(deserialize = "T: Deserialize<'de>"))]
        struct List<T> {
            #[serde(deserialize_with = "deserialize_vec")]
            items: Vec<T>,
        }"#};

    assert_analyzed_source_code(source_code, result, "rust")
}

#[test]
fn test_idempotency_within_nested_scope() {
    let source_code = indoc! {"
        # [TODO] Post
        class Post(models.Model):
            user = models.ForeignKey(User)

            # [TODO] Post > Meta
            class Meta:
                table_name = 'posts'

            # [TODO] Post > count
            @staticmethod
            def count(cls):
                return cls.count

            # [TODO] Post > author
            def author(self):
                return self.user
                
        # [TODO] Comment
        class Comment(models.Model):
            user = models.ForeignKey(User)

            # [TODO] Comment > Meta
            class Meta:
                table_name = 'comments'

            # [TODO] Comment > count
            @staticmethod
            def count(cls):
                return cls.count

            # [TODO] Comment > author
            def author(self):
                return self.user"};

    let result = indoc! {"
        # [TODO] Post
        class Post(models.Model):
            user = models.ForeignKey(User)

            # [TODO] Post > Meta
            class Meta:
                table_name = 'posts'

            # [TODO] Post > count
            @staticmethod
            def count(cls):
                return cls.count

            # [TODO] Post > author
            def author(self):
                return self.user
                
        # [TODO] Comment
        class Comment(models.Model):
            user = models.ForeignKey(User)

            # [TODO] Comment > Meta
            class Meta:
                table_name = 'comments'

            # [TODO] Comment > count
            @staticmethod
            def count(cls):
                return cls.count

            # [TODO] Comment > author
            def author(self):
                return self.user"};

    assert_analyzed_source_code(source_code, result, "python")
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
        /// [TODO] tests
        #[cfg(test)]
        mod tests {
            use super::*;

            /// [TODO] tests > test_foo
            #[test]
            fn test_foo() {
                assert_eq!(foo(), 1);
            }
        }"};

    assert_analyzed_source_code(source_code, result, "rust")
}

#[test]
fn test_ignore_doc_macro() {
    let source_code = indoc! {r#"
        #[doc = "This is a doc comment"]
        fn foo() {
            println!("foo");
        }"#};

    let result = indoc! {r#"
        /// [TODO] foo
        #[doc = "This is a doc comment"]
        fn foo() {
            println!("foo");
        }"#};

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
    /// [TODO] RangeFactory
    pub trait RangeFactory {
        fn from_node(node: Node) -> Range;
    }

    /// [TODO] RangeFactory
    impl RangeFactory for Range {
        /// [TODO] RangeFactory > from_node
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
    /// [TODO] tree_sitter_extended
    mod tree_sitter_extended {
        /// [TODO] tree_sitter_extended > RangeFactory
        pub trait RangeFactory {
            fn from_node(node: Node) -> Range;
        }

        /// [TODO] tree_sitter_extended > RangeFactory
        impl RangeFactory for Range {
            /// [TODO] tree_sitter_extended > RangeFactory > from_node
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
