use crate::integration_test::assert_analyzed_source_code;
use indoc::indoc;

/// https://github.com/RustPython/RustPython/blob/bdb0c8f64557e0822f0bcfd63defbad54625c17a/jit/src/lib.rs#L10-L28
#[test]
fn test_declaring_enum_with_stacked_attribute() {
    let source_code = indoc! {r#"
    #[derive(Debug, thiserror::Error)]
    #[non_exhaustive]
    pub enum JitCompileError {
        #[error("function can't be jitted")]
        NotSupported,
        #[error("bad bytecode")]
        BadBytecode,
        #[error("error while compiling to machine code: {0}")]
        CraneliftError(#[from] ModuleError),
    }

    #[derive(Debug, thiserror::Error, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum JitArgumentError {
        #[error("argument is of wrong type")]
        ArgumentTypeMismatch,
        #[error("wrong number of arguments")]
        WrongNumberOfArguments,
    }"#};

    let result = indoc! {r#"
    /// [TODO] JitCompileError
    #[derive(Debug, thiserror::Error)]
    #[non_exhaustive]
    pub enum JitCompileError {
        #[error("function can't be jitted")]
        NotSupported,
        #[error("bad bytecode")]
        BadBytecode,
        #[error("error while compiling to machine code: {0}")]
        CraneliftError(#[from] ModuleError),
    }

    /// [TODO] JitArgumentError
    #[derive(Debug, thiserror::Error, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum JitArgumentError {
        #[error("argument is of wrong type")]
        ArgumentTypeMismatch,
        #[error("wrong number of arguments")]
        WrongNumberOfArguments,
    }"#};

    assert_analyzed_source_code(source_code, result, "rust")
}

/// https://github.com/RustPython/RustPython/blob/bdb0c8f64557e0822f0bcfd63defbad54625c17a/vm/src/compiler.rs#L5C1-L6
#[test]
fn test_macro_above_use_declaration_should_be_ignored() {
    let source_code = indoc! { r#"
    #[cfg(feature = "rustpython-compiler")]
    use rustpython_compiler::*;"#};

    let result = indoc! { r#"
    #[cfg(feature = "rustpython-compiler")]
    use rustpython_compiler::*;"#};

    assert_analyzed_source_code(source_code, result, "rust")
}

/// https://github.com/RustPython/RustPython/blob/bdb0c8f64557e0822f0bcfd63defbad54625c17a/wasm/lib/src/js_module.rs#L24-L55
#[test]
fn test_macro_above_extern_c_module() {
    let source_code = indoc! { r#"
    #[wasm_bindgen(inline_js = "
    export function has_prop(target, prop) { return prop in Object(target); }
    export function get_prop(target, prop) { return target[prop]; }
    export function set_prop(target, prop, value) { target[prop] = value; }
    export function type_of(a) { return typeof a; }
    export function instance_of(lhs, rhs) { return lhs instanceof rhs; }
    ")]
    extern "C" {
        #[wasm_bindgen(catch)]
        fn has_prop(target: &JsValue, prop: &JsValue) -> Result<bool, JsValue>;
        #[wasm_bindgen(catch)]
        fn get_prop(target: &JsValue, prop: &JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        fn set_prop(target: &JsValue, prop: &JsValue, value: &JsValue) -> Result<(), JsValue>;
        #[wasm_bindgen]
        fn type_of(a: &JsValue) -> String;
        #[wasm_bindgen(catch)]
        fn instance_of(lhs: &JsValue, rhs: &JsValue) -> Result<bool, JsValue>;
    }"#};

    let result = indoc! { r#"
    #[wasm_bindgen(inline_js = "
    export function has_prop(target, prop) { return prop in Object(target); }
    export function get_prop(target, prop) { return target[prop]; }
    export function set_prop(target, prop, value) { target[prop] = value; }
    export function type_of(a) { return typeof a; }
    export function instance_of(lhs, rhs) { return lhs instanceof rhs; }
    ")]
    extern "C" {
        #[wasm_bindgen(catch)]
        fn has_prop(target: &JsValue, prop: &JsValue) -> Result<bool, JsValue>;
        #[wasm_bindgen(catch)]
        fn get_prop(target: &JsValue, prop: &JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        fn set_prop(target: &JsValue, prop: &JsValue, value: &JsValue) -> Result<(), JsValue>;
        #[wasm_bindgen]
        fn type_of(a: &JsValue) -> String;
        #[wasm_bindgen(catch)]
        fn instance_of(lhs: &JsValue, rhs: &JsValue) -> Result<bool, JsValue>;
    }"#};

    assert_analyzed_source_code(source_code, result, "rust")
}
