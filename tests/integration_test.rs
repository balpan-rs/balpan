#[cfg(test)]
mod integration_test {
    mod python_test;
    mod rust_test;

    use balpan::analyzer::{Analyzer, Traversable};
    use balpan::grammar::{build_grammars, fetch_grammars};
    use balpan::language::Language;

    pub fn assert_analyzed_source_code(source_code: &str, expected: &str, language: &str) {
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

        let actual: String = string_vector.join("\n");

        if actual != expected {
            println!("expected: {}\n\n", expected);
            println!("actual: {}\n\n", actual);
        }

        assert_eq!(expected, actual);
    }
}
