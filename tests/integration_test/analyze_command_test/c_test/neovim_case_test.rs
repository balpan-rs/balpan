#[cfg(test)]
mod neovim_case_test {
    use crate::integration_test::assert_analyzed_source_code;
    use indoc::indoc;

    #[test]
    fn test_function_definition() {
        let source_code = indoc! { r#"
        static OptVal object_as_optval(Object o, bool *error)
        {
          switch (o.type) {
          case kObjectTypeNil:
            return NIL_OPTVAL;
          case kObjectTypeBoolean:
            return BOOLEAN_OPTVAL(o.data.boolean);
          case kObjectTypeInteger:
            return NUMBER_OPTVAL(o.data.integer);
          case kObjectTypeString:
            return STRING_OPTVAL(o.data.string);
          default:
            *error = true;
            return NIL_OPTVAL;
          }
        }"#};

        let result = indoc! { r#"
        /// [TODO] object_as_optval
        static OptVal object_as_optval(Object o, bool *error)
        {
          switch (o.type) {
          case kObjectTypeNil:
            return NIL_OPTVAL;
          case kObjectTypeBoolean:
            return BOOLEAN_OPTVAL(o.data.boolean);
          case kObjectTypeInteger:
            return NUMBER_OPTVAL(o.data.integer);
          case kObjectTypeString:
            return STRING_OPTVAL(o.data.string);
          default:
            *error = true;
            return NIL_OPTVAL;
          }
        }"#};

        assert_analyzed_source_code(source_code, result, "cpp");
    }

    #[test]
    fn test_function_definition_with_conditional_compilation() {
        let source_code = indoc! { r#"
        int path_is_absolute(const char *fname)
        {
        #ifdef MSWIN
          if (*fname == NUL) {
            return false;
          }
          // A name like "d:/foo" and "//server/share" is absolute
          return ((isalpha((uint8_t)fname[0]) && fname[1] == ':' && vim_ispathsep_nocolon(fname[2]))
                  || (vim_ispathsep_nocolon(fname[0]) && fname[0] == fname[1]));
        #else
          // UNIX: This just checks if the file name starts with '/' or '~'.
          return *fname == '/' || *fname == '~';
        #endif
        }"#};

        let result = indoc! { r#"
        /// [TODO] path_is_absolute
        int path_is_absolute(const char *fname)
        {
        #ifdef MSWIN
          if (*fname == NUL) {
            return false;
          }
          // A name like "d:/foo" and "//server/share" is absolute
          return ((isalpha((uint8_t)fname[0]) && fname[1] == ':' && vim_ispathsep_nocolon(fname[2]))
                  || (vim_ispathsep_nocolon(fname[0]) && fname[0] == fname[1]));
        #else
          // UNIX: This just checks if the file name starts with '/' or '~'.
          return *fname == '/' || *fname == '~';
        #endif
        }"#};

        assert_analyzed_source_code(source_code, result, "cpp");
    }
}
