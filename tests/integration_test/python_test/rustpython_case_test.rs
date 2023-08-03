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

    /// Test stdlib
    ///
    #[test]
    fn test_class_definition() {
        let source_code = indoc! {"
        class FeedParser:
            \"\"\"A feed-style parser of email.\"\"\"

            def __init__(self, _factory=None, *, policy=compat32):
                \"\"\"_factory is called with no arguments to create a new message obj

                The policy keyword specifies a policy object that controls a number of
                aspects of the parser's operation.  The default policy maintains
                backward compatibility.

                \"\"\"
                self.policy = policy
                self._old_style_factory = False
                if _factory is None:
                    if policy.message_factory is None:
                        from email.message import Message
                        self._factory = Message
                    else:
                        self._factory = policy.message_factory
                else:
                    self._factory = _factory
                    try:
                        _factory(policy=self.policy)
                    except TypeError:
                        # Assume this is an old-style factory
                        self._old_style_factory = True
                self._input = BufferedSubFile()
                self._msgstack = []
                self._parse = self._parsegen().__next__
                self._cur = None
                self._last = None
                self._headersonly = False
                
            # Non-public interface for supporting Parser's headersonly flag
            def _set_headersonly(self):
                self._headersonly = True

            def feed(self, data):
                \"\"\"Push more data into the parser.\"\"\"
                self._input.push(data)
                self._call_parse()

            def _call_parse(self):
                try:
                    self._parse()
                except StopIteration:
                    pass"};

        let result = indoc! {"
        # [TODO]
        class FeedParser:
            \"\"\"A feed-style parser of email.\"\"\"

            # [TODO]
            def __init__(self, _factory=None, *, policy=compat32):
                \"\"\"_factory is called with no arguments to create a new message obj

                The policy keyword specifies a policy object that controls a number of
                aspects of the parser's operation.  The default policy maintains
                backward compatibility.

                \"\"\"
                self.policy = policy
                self._old_style_factory = False
                if _factory is None:
                    if policy.message_factory is None:
                        from email.message import Message
                        self._factory = Message
                    else:
                        self._factory = policy.message_factory
                else:
                    self._factory = _factory
                    try:
                        _factory(policy=self.policy)
                    except TypeError:
                        # Assume this is an old-style factory
                        self._old_style_factory = True
                self._input = BufferedSubFile()
                self._msgstack = []
                self._parse = self._parsegen().__next__
                self._cur = None
                self._last = None
                self._headersonly = False
                
            # Non-public interface for supporting Parser's headersonly flag
            # [TODO]
            def _set_headersonly(self):
                self._headersonly = True

            # [TODO]
            def feed(self, data):
                \"\"\"Push more data into the parser.\"\"\"
                self._input.push(data)
                self._call_parse()

            # [TODO]
            def _call_parse(self):
                try:
                    self._parse()
                except StopIteration:
                    pass"};

        assert_analyzed_source_code(source_code, result, "python")
    }
}
