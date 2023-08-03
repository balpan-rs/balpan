#[cfg(test)]
mod python_dependency_injector_case_test {
    use indoc::indoc;
    use crate::integration_test::assert_analyzed_source_code;

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
