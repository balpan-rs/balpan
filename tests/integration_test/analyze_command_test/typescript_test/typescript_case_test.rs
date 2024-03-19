use crate::integration_test::assert_analyzed_source_code;
use indoc::indoc;

#[test]
fn test_typescript_export_functions() {
    let source_code = indoc! {r#"
    export function parseBindingIdentifier(privateIdentifierDiagnosticMessage?: DiagnosticMessage) {
        return createIdentifier(isBindingIdentifier(), /*diagnosticMessage*/ undefined, privateIdentifierDiagnosticMessage);
    }
    export function parseIdentifier(diagnosticMessage?: DiagnosticMessage, privateIdentifierDiagnosticMessage?: DiagnosticMessage): Identifier {
        return createIdentifier(isIdentifier(), diagnosticMessage, privateIdentifierDiagnosticMessage);
    }
    export function parseIdentifierName(diagnosticMessage?: DiagnosticMessage): Identifier {
        return createIdentifier(tokenIsIdentifierOrKeyword(token()), diagnosticMessage);
    }"#};

    let expected = indoc! {r#"
    // [TODO] parseBindingIdentifier
    export function parseBindingIdentifier(privateIdentifierDiagnosticMessage?: DiagnosticMessage) {
        return createIdentifier(isBindingIdentifier(), /*diagnosticMessage*/ undefined, privateIdentifierDiagnosticMessage);
    }
    // [TODO] parseIdentifier
    export function parseIdentifier(diagnosticMessage?: DiagnosticMessage, privateIdentifierDiagnosticMessage?: DiagnosticMessage): Identifier {
        return createIdentifier(isIdentifier(), diagnosticMessage, privateIdentifierDiagnosticMessage);
    }
    // [TODO] parseIdentifierName
    export function parseIdentifierName(diagnosticMessage?: DiagnosticMessage): Identifier {
        return createIdentifier(tokenIsIdentifierOrKeyword(token()), diagnosticMessage);
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}

#[test]
fn test_typescript_functions() {
    let source_code = indoc! {r#"
    function setContextFlag(val: boolean, flag: NodeFlags) {
        if (val) {
            contextFlags |= flag;
        }
        else {
            contextFlags &= ~flag;
        }
    }

    function setDisallowInContext(val: boolean) {
        setContextFlag(val, NodeFlags.DisallowInContext);
    }

    function setYieldContext(val: boolean) {
        setContextFlag(val, NodeFlags.YieldContext);
    }

    function setDecoratorContext(val: boolean) {
        setContextFlag(val, NodeFlags.DecoratorContext);
    }

    function setAwaitContext(val: boolean) {
        setContextFlag(val, NodeFlags.AwaitContext);
    }"#};

    let expected = indoc! {r#"
    // [TODO] setContextFlag
    function setContextFlag(val: boolean, flag: NodeFlags) {
        if (val) {
            contextFlags |= flag;
        }
        else {
            contextFlags &= ~flag;
        }
    }

    // [TODO] setDisallowInContext
    function setDisallowInContext(val: boolean) {
        setContextFlag(val, NodeFlags.DisallowInContext);
    }

    // [TODO] setYieldContext
    function setYieldContext(val: boolean) {
        setContextFlag(val, NodeFlags.YieldContext);
    }

    // [TODO] setDecoratorContext
    function setDecoratorContext(val: boolean) {
        setContextFlag(val, NodeFlags.DecoratorContext);
    }

    // [TODO] setAwaitContext
    function setAwaitContext(val: boolean) {
        setContextFlag(val, NodeFlags.AwaitContext);
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}
