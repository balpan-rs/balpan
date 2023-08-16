use crate::language::Language;

pub enum CommentToken {
    Rust,
    Python,
    Other,
}

impl CommentToken {
    pub fn from_language(language: &Language) -> Self {
        match language {
            Language::Rust => CommentToken::Rust,
            Language::Python => CommentToken::Python,
            _ => CommentToken::Other,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            CommentToken::Rust => "/// [TODO]",
            CommentToken::Python => "# [TODO]",
            CommentToken::Other => "// [TODO]",
        }
    }
}
