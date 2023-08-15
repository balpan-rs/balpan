#[cfg(test)]
mod anyhow_case_test {
    use crate::integration_test::assert_analyzed_source_code;
    use indoc::indoc;

    #[test]
    fn test_declaring_error_enum_with_macro() {
        let source_code = indoc! {r#"
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
        }"#};

        let result = indoc! {r#"
        use thiserror::Error;

        /// [TODO] FormatError
        #[derive(Error, Debug)]
        pub enum FormatError {
            #[error("Invalid header (expected {expected:?}, got {found:?})")]
            InvalidHeader {
                expected: String,
                found: String,
            },
            #[error("Missing attribute: {0}")]
            MissingAttribute(String),
        }"#};

        assert_analyzed_source_code(source_code, result, "rust")
    }
}
