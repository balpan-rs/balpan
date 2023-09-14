use indoc::indoc;

use crate::integration_test::assert_analyzed_source_code;

#[test]
#[ignore = "TODO: arrow function"]
fn test_arrow_function() {
    let source_code = indoc! {r#"
    export const parse = (source) =>
	code_red.parse(source, {
		sourceType: 'module',
		ecmaVersion: 13,
		locations: true
	});

    /**
     * @param {string} source
     * @param {number} index
     */
    export const parse_expression_at = (source, index) =>
        code_red.parseExpressionAt(source, index, {
            sourceType: 'module',
            ecmaVersion: 13,
            locations: true
        });"#};

    let expected = indoc! {r#"
    // [TODO] parse
    export const parse = (source) =>
    code_red.parse(source, {
        sourceType: 'module',
        ecmaVersion: 13,
        locations: true
    });

    /**
     * @param {string} source
     * @param {number} index
     */
    // [TODO] parse_expression_at
    export const parse_expression_at = (source, index) =>
        code_red.parseExpressionAt(source, index, {
            sourceType: 'module',
            ecmaVersion: 13,
            locations: true
        });"#};

    assert_analyzed_source_code(source_code, expected, "javascript")
}