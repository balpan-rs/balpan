use crate::language::Language;

pub enum CommentToken {
    TripleSlashTODO,
    DoubleSlashTODO,
    HashTODO,
    Other,
}

impl CommentToken {
    pub fn from_language(language: &Language) -> Self {
        match language {
            Language::Rust | Language::Cpp => CommentToken::TripleSlashTODO,
            Language::Python | Language::Ruby => CommentToken::HashTODO,
            Language::JavaScript | Language::TypeScript => CommentToken::DoubleSlashTODO,
            _ => CommentToken::Other,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            CommentToken::TripleSlashTODO => "/// [TODO]",
            CommentToken::DoubleSlashTODO => "// [TODO]",
            CommentToken::HashTODO => "# [TODO]",
            CommentToken::Other => "",
        }
    }
}
