use indoc::indoc;
use crate::integration_test::assert_analyzed_source_code;

#[test]
fn test_async_function_expression() {
    let source_code = indoc! {r#"
    async function foo (){
        const dddd = await asyncBusby(22);
        console.log(dddd);
    }"#};

    let expected = indoc! {r#"
    // [TODO] foo
    async function foo (){
        const dddd = await asyncBusby(22);
        console.log(dddd);
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}

#[test]
#[ignore = "TODO: Support arrow function"]
fn test_async_arrow_function() {
    let source_code = indoc! {r#"
    const foo = async () => {
        const dddd = await asyncBusby(22);
        console.log(dddd);
    }"#};

    let expected = indoc! {r#"
    // [TODO] foo
    const foo = async () => {
        const dddd = await asyncBusby(22);
        console.log(dddd);
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}