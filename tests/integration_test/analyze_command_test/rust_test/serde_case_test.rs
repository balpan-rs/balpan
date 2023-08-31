use crate::integration_test::assert_analyzed_source_code;
use indoc::indoc;

#[test]
fn test_several_impl_declaration() {
    let source_code = indoc! {"
    impl PartialEq<Symbol> for Ident {
        fn eq(&self, word: &Symbol) -> bool {
            self == word.0
        }
    }

    impl<'a> PartialEq<Symbol> for &'a Ident {
        fn eq(&self, word: &Symbol) -> bool {
            *self == word.0
        }
    }

    impl PartialEq<Symbol> for Path {
        fn eq(&self, word: &Symbol) -> bool {
            self.is_ident(word.0)
        }
    }"};

    let result = indoc! {"
    /// [TODO] PartialEq<Symbol>
    impl PartialEq<Symbol> for Ident {
        /// [TODO] PartialEq<Symbol> > eq
        fn eq(&self, word: &Symbol) -> bool {
            self == word.0
        }
    }

    /// [TODO] PartialEq<Symbol>
    impl<'a> PartialEq<Symbol> for &'a Ident {
        /// [TODO] PartialEq<Symbol> > eq
        fn eq(&self, word: &Symbol) -> bool {
            *self == word.0
        }
    }

    /// [TODO] PartialEq<Symbol>
    impl PartialEq<Symbol> for Path {
        /// [TODO] PartialEq<Symbol> > eq
        fn eq(&self, word: &Symbol) -> bool {
            self.is_ident(word.0)
        }
    }"};

    assert_analyzed_source_code(source_code, result, "rust")
}


/// https://github.com/serde-rs/serde/blob/7b548db91ed7da81a5c0ddbd6f6f21238aacfebe/serde/src/lib.rs#L155-L156
#[test]
fn test_macro_above_extern_crate_declaration_should_be_ignored() {
    let source_code = indoc! { r#"
    #[cfg(feature = "alloc")]
     extern crate alloc;"#};

    let result = indoc! { r#"
    #[cfg(feature = "alloc")]
     extern crate alloc;"#};

    assert_analyzed_source_code(source_code, result, "rust");
}

/// https://github.com/serde-rs/serde/blob/7b548db91ed7da81a5c0ddbd6f6f21238aacfebe/precompiled/bin/main.rs#L11-L12
#[test]
fn test_macro_above_static_variable_should_be_ignored() {
    let source_code = indoc! {r#"
    #[global_allocator]
    static ALLOCATOR: MonotonicAllocator = MonotonicAllocator;"#};

    let result = indoc! {r#"
    #[global_allocator]
    static ALLOCATOR: MonotonicAllocator = MonotonicAllocator;"#};

    assert_analyzed_source_code(source_code, result, "rust")
}

/// https://github.com/serde-rs/serde/blob/7b548db91ed7da81a5c0ddbd6f6f21238aacfebe/serde/src/de/impls.rs#L1783-L1793
#[test]
fn test_macro_above_macro_invocation_should_be_ignored() {
    let source_code = indoc! { r#" 
    #[cfg(any(feature = "std", feature = "alloc"))]
    forwarded_impl!((T), Box<T>, Box::new);
     
    #[cfg(any(feature = "std", feature = "alloc"))]
    forwarded_impl!((T), Box<[T]>, Vec::into_boxed_slice);
     
    #[cfg(any(feature = "std", feature = "alloc"))]
    forwarded_impl!((), Box<str>, String::into_boxed_str);
     
    #[cfg(all(feature = "std", any(unix, windows)))]
    forwarded_impl!((), Box<OsStr>, OsString::into_boxed_os_str);"#};

    let result = indoc! { r#" 
    #[cfg(any(feature = "std", feature = "alloc"))]
    forwarded_impl!((T), Box<T>, Box::new);
     
    #[cfg(any(feature = "std", feature = "alloc"))]
    forwarded_impl!((T), Box<[T]>, Vec::into_boxed_slice);
     
    #[cfg(any(feature = "std", feature = "alloc"))]
    forwarded_impl!((), Box<str>, String::into_boxed_str);
     
    #[cfg(all(feature = "std", any(unix, windows)))]
    forwarded_impl!((), Box<OsStr>, OsString::into_boxed_os_str);"#};

    assert_analyzed_source_code(source_code, result, "rust")
}

/// https://github.com/serde-rs/serde/blob/7b548db91ed7da81a5c0ddbd6f6f21238aacfebe/serde/src/de/mod.rs#L119-L126
#[test]
fn test_ignore_mod_items_in_a_row() {
    let source_code = indoc! { r#"
     pub mod value;
     
     #[cfg(not(no_integer128))]
     mod format;
     mod ignored_any;
     mod impls;
     pub(crate) mod size_hint;
     mod utf8;"#};

    let result = indoc! { r#"
     pub mod value;
     
     #[cfg(not(no_integer128))]
     mod format;
     mod ignored_any;
     mod impls;
     pub(crate) mod size_hint;
     mod utf8;"#};

     assert_analyzed_source_code(source_code, result, "rust");
}
