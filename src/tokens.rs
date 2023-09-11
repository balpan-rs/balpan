use crate::language::Language;

pub enum CommentToken {
    Rust,
    Python,
    Ruby,
    Cpp,
    TypeScript,
    Other,
}

impl CommentToken {
    pub fn from_language(language: &Language) -> Self {
        match language {
            Language::Rust => CommentToken::Rust,
            Language::Python => CommentToken::Python,
            Language::Ruby => CommentToken::Ruby,
            Language::Cpp => CommentToken::Cpp,
            Language::TypeScript => CommentToken::TypeScript,
            _ => CommentToken::Other,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            CommentToken::Cpp | CommentToken::Rust => "/// [TODO]",
            CommentToken::Python | CommentToken::Ruby => "# [TODO]",
            CommentToken::TypeScript | _ => "// [TODO]",
        }
    }
}
