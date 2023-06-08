use anyhow::Result;

use balpan::grammar::fetch_grammars;
use balpan::utils::{get_git_repo_root, list_available_files};

fn main() -> Result<()> {
    if let Some(repo_root) = get_git_repo_root() {
        let _ = list_available_files(&repo_root);
        println!("Git repository root: {}", repo_root);
    } else {
        println!("Not inside a Git repository.");
    }
    fetch_grammars()
}
