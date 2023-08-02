#[cfg(test)]
mod python_dependency_injector_case_test {
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
    fn test_decorated_definition() {
        let source_code = indoc! {"
        @app.route(\"/\")
        @inject
        def index(service: Service = Provide[Container.service]):
            result = service.process()
            return jsonify({\"result\": result})"};
        
        let result = indoc! {"
        # [TODO]
        @app.route(\"/\")
        @inject
        def index(service: Service = Provide[Container.service]):
            result = service.process()
            return jsonify({\"result\": result})"};

        assert_analyzed_source_code(source_code, result, "python")
    }

    #[test]
    fn test_decorated_async_function_definition() {
        let source_code = indoc! {"      
        @inject
        async def async_injection(
                resource1: object = Provide[\"resource1\"],
                resource2: object = Provide[\"resource2\"],
        ):
            return resource1, resource2

        @inject
        async def async_injection_with_closing(
                resource1: object = Closing[Provide[\"resource1\"]],
                resource2: object = Closing[Provide[\"resource2\"]],
        ):
            return resource1, resource2"};

        let result = indoc! {"      
        # [TODO]
        @inject
        async def async_injection(
                resource1: object = Provide[\"resource1\"],
                resource2: object = Provide[\"resource2\"],
        ):
            return resource1, resource2

        # [TODO]
        @inject
        async def async_injection_with_closing(
                resource1: object = Closing[Provide[\"resource1\"]],
                resource2: object = Closing[Provide[\"resource2\"]],
        ):
            return resource1, resource2"};

        assert_analyzed_source_code(source_code, result, "python")
    }
}
