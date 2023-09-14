#[derive(PartialEq)]
pub enum Language {
    Rust,
    Python,
    Ruby,
    Cpp,
    TypeScript,
    JavaScript,
    Other(String),
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::Ruby => "ruby",
            Self::Cpp => "cpp",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
            Self::Other(ref language) => language.as_str(),
        }
    }

    #[inline]
    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "rs" => Self::Rust,
            "py" => Self::Python,
            "rb" => Self::Ruby,
            "cpp" => Self::Cpp,
            "h" => Self::Cpp,
            "hpp" => Self::Cpp,
            "ts" => Self::TypeScript,
            "js" => Self::JavaScript,
            other_extension => Self::Other(other_extension.to_string()),
        }
    }

    /// language specific tree-sitter node types
    pub fn top_level_node_type(&self) -> &str {
        match self {
            Language::Rust => "source_file",
            Language::Python => "module",
            Language::Ruby 
            | Language::JavaScript 
            | Language::TypeScript  => "program",
            Language::Cpp => "translation_unit",
            _ => "",
        }
    }

    pub fn decorator_node_type(&self) -> &str {
        match self {
            Language::Rust => "attribute_item",
            Language::Python 
            | Language::Ruby 
            | Language::Cpp => "null",
            Language::TypeScript | Language::JavaScript => "decorator",
            _ => "",
        }
    }

    pub fn comment_node_type(&self) -> &str {
        match self {
            Language::Rust => "line_comment",
            Language::Python
            | Language::Ruby
            | Language::Cpp
            | Language::TypeScript
            | Language::JavaScript => "comment",
            _ => "",
        }
    }

    pub fn scannable_node_types(&self) -> Vec<&str> {
        let mut scannable = self.ignorable_node_types();
        let mut commentable = self.commentable_node_types();
        scannable.append(&mut commentable);
        scannable
    }

    pub fn ignorable_node_types(&self) -> Vec<&str> {
        match self {
            Language::Rust => vec![
                "type_item",
                "static_item",
                "extern_crate_declaration",
                "const_item",
                "use_declaration",
                "expression_statement",
                "macro_invocation",
                "foreign_mod_item", // extern "C"
            ],
            Language::TypeScript | Language::JavaScript => {
                vec!["string_fragment", "import_specifier", "named_imports"]
            }
            _ => vec![],
        }
    }

    pub fn commentable_node_types(&self) -> Vec<&str> {
        match self {
            Language::Rust => vec![
                "attribute_item",
                "mod_item",
                "enum_item",
                "impl_item",
                "function_item",
                "struct_item",
                "trait_item",
                "macro_definition",
            ],
            Language::Python => vec![
                "class_definition",
                "function_definition",
                "decorated_definition",
            ],
            Language::Ruby => vec!["class", "method", "function", "module"],
            Language::Cpp => vec![
                "namespace_definition",
                "function_definition",
                "class_specifier",
            ],
            Language::TypeScript | Language::JavaScript => vec![
                "enum_declaration",
                "function_declaration",
                "class_declaration",
                "method_definition",
                "interface_declaration",
                "export_statement",
                // "variable_declaration",
                "expression_statement", // namespace
            ],
            _ => vec![],
        }
    }

    pub fn nested_traversable_symbols(&self) -> Vec<&str> {
        match self {
            Language::Rust => vec!["mod_item", "impl_item"],
            Language::Python => vec!["class_definition"],
            Language::Ruby => vec!["class", "module"],
            Language::Cpp => vec!["namespace_definition", "class_specifier"],
            Language::TypeScript 
            | Language::JavaScript => vec![
                "class_declaration",
                "expression_statement",
                "internal_module",
            ],
            _ => vec![],
        }
    }
}

impl From<&str> for Language {
    fn from(language_name: &str) -> Self {
        match language_name {
            "rust" => Self::Rust,
            "python" => Self::Python,
            "ruby" => Self::Ruby,
            "cpp" => Self::Cpp,
            "typescript" => Self::TypeScript,
            "javascript" => Self::JavaScript,
            other_language => Self::Other(other_language.to_string()),
        }
    }
}
