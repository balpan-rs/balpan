use crate::language::Language;

pub enum CommentToken {
    Rust,
    Python,
    Ruby,
    Cpp,
    Other,
}

impl CommentToken {
    pub fn from_language(language: &Language) -> Self {
        match language {
            Language::Rust => CommentToken::Rust,
            Language::Python => CommentToken::Python,
            Language::Ruby => CommentToken::Ruby,
            Language::Cpp => CommentToken::Cpp,
            _ => CommentToken::Other,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            CommentToken::Rust => "/// [TODO]",
            CommentToken::Python => "# [TODO]",
            CommentToken::Ruby => "# [TODO]",
            CommentToken::Cpp => "/// [TODO]",
            CommentToken::Other => "// [TODO]",
        }
    }
}
