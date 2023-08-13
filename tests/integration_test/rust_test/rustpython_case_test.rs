#[cfg(test)]
mod rustpython_case_test {
    use indoc::indoc;
    use crate::integration_test::assert_analyzed_source_code;

    #[test]
    fn test_declaring_enum_with_stacked_attribute() {
        let source_code = indoc! {"
        #[derive(Debug, thiserror::Error)]
        #[non_exhaustive]
        pub enum JitCompileError {
            #[error(\"function can't be jitted\")]
            NotSupported,
            #[error(\"bad bytecode\")]
            BadBytecode,
            #[error(\"error while compiling to machine code: {0}\")]
            CraneliftError(#[from] ModuleError),
        }

        #[derive(Debug, thiserror::Error, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum JitArgumentError {
            #[error(\"argument is of wrong type\")]
            ArgumentTypeMismatch,
            #[error(\"wrong number of arguments\")]
            WrongNumberOfArguments,
        }"};

        let result = indoc! {"
        /// [TODO] JitCompileError
        #[derive(Debug, thiserror::Error)]
        #[non_exhaustive]
        pub enum JitCompileError {
            #[error(\"function can't be jitted\")]
            NotSupported,
            #[error(\"bad bytecode\")]
            BadBytecode,
            #[error(\"error while compiling to machine code: {0}\")]
            CraneliftError(#[from] ModuleError),
        }

        /// [TODO] JitArgumentError
        #[derive(Debug, thiserror::Error, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum JitArgumentError {
            #[error(\"argument is of wrong type\")]
            ArgumentTypeMismatch,
            #[error(\"wrong number of arguments\")]
            WrongNumberOfArguments,
        }"};

        assert_analyzed_source_code(source_code, result, "rust")
    }
}
