
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