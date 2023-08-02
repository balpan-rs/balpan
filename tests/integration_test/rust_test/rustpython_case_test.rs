#[cfg(test)]
mod rustpython_case_test {
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
            // .map( |str| { str } )
            // .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(expected, actual);
    }

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
        /// [TODO]
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

        /// [TODO]
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
