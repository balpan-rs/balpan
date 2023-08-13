#[cfg(test)]
mod serde_case_test {
    use indoc::indoc;
    use crate::integration_test::assert_analyzed_source_code;

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
}
