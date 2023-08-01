#[cfg(test)]
mod anyhow_case_test {
    use indoc::indoc;
    use balpan::analyzer::{Analyzer,Traversable};
    use balpan::grammar::{fetch_grammars, build_grammars};

    fn assert_analyzed_source_code(source_code: &str, expected: &str) {
        fetch_grammars().unwrap();
        build_grammars(None).unwrap();

        let analyzer = Analyzer {
            source_code: source_code.to_string()
        };

        let writer_queue = &analyzer.analyze();
        let mut string_vector = vec![];

        for line in writer_queue {
            string_vector.push(*line);
        }


        let actual: String = string_vector
            .iter()
            .map( |str| { *str } )
            .collect::<Vec<&str>>()
            .join("\n");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_declaring_error_enum_with_macro() {
        let source_code = indoc! {"
        use thiserror::Error;

        #[derive(Error, Debug)]
        pub enum FormatError {
            #[error(\"Invalid header (expected {expected:?}, got {found:?})\")]
            InvalidHeader {
                expected: String,
                found: String,
            },
            #[error(\"Missing attribute: {0}\")]
            MissingAttribute(String),
        }"};

        let result = indoc! {"
        use thiserror::Error;

        /// [TODO]
        #[derive(Error, Debug)]
        pub enum FormatError {
            #[error(\"Invalid header (expected {expected:?}, got {found:?})\")]
            InvalidHeader {
                expected: String,
                found: String,
            },
            #[error(\"Missing attribute: {0}\")]
            MissingAttribute(String),
        }"};

        assert_analyzed_source_code(source_code, result)
    }
}
