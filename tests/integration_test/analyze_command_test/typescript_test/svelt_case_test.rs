use indoc::indoc;

use crate::integration_test::assert_analyzed_source_code;

#[test]
fn test_interface_and_type_extends_with_exports() {
    let source_code = indoc! {r#"
    interface BaseNode {
        start: number;
        end: number;
        type: string;
        children?: TemplateNode[];
        [prop_name: string]: any;
    }

    export type DirectiveType =
    | 'Action'
    | 'Animation'
    | 'Binding'
    | 'Class'
    | 'StyleDirective'
    | 'EventHandler'
    | 'Let'
    | 'Ref'
    | 'Transition';

    export interface BaseDirective extends BaseNode {
        type: DirectiveType;
        name: string;
    }"#};

    let expected = indoc! {r#"
    // [TODO] BaseNode
    interface BaseNode {
        start: number;
        end: number;
        type: string;
        children?: TemplateNode[];
        [prop_name: string]: any;
    }

    // [TODO] DirectiveType
    export type DirectiveType =
    | 'Action'
    | 'Animation'
    | 'Binding'
    | 'Class'
    | 'StyleDirective'
    | 'EventHandler'
    | 'Let'
    | 'Ref'
    | 'Transition';

    // [TODO] BaseDirective
    export interface BaseDirective extends BaseNode {
        type: DirectiveType;
        name: string;
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}