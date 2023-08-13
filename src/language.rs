#[derive(PartialEq)]
pub enum Language{
    Rust,
    Python,
    Other(String),
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::Other(language) => language.as_str(),
        }
    }

    #[inline]
    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "rs" => Self::Rust,
            "py" => Self::Python,
            other_extension => Self::Other(other_extension.to_string())
        }
    }
}

impl From<&str> for Language{
  fn from(language_name: &str) -> Self {
      match language_name {
          "rust" => Self::Rust,
          "python" => Self::Python,
          other_language => Self::Other(other_language.to_string()),
      }
  }
}
