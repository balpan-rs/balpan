use std::env;

use std::fs::File;

use git2::Repository;
use ignore::WalkBuilder;

pub fn get_current_repository() -> Option<Repository> {
    let current_dir = env::current_dir().ok()?;
    let repo = Repository::discover(current_dir).ok()?;

    Some(repo)
}

// pub fn get_git_repo_root() -> Option<String> {
//     let repo = get_current_repository().ok()?;
//     let repo_root = repo.workdir()?.to_string_lossy().to_string();
//
//     Some(repo_root)
// }

pub fn list_available_files(repo_path: &str) -> Vec<String> {
    let mut result = Vec::new();
    let walker = WalkBuilder::new(repo_path)
        .hidden(true)
        .git_ignore(true)
        .parents(false)
        .build();

    // Traverse the directory with gitignore rules applied
    for entry in walker.flatten() {
        // Skip directories
        if entry.file_type().expect(".").is_dir() {
            continue;
        }

        // Open each file and process it
        if let Ok(_file) = File::open(entry.path()) {
            result.push(String::from(entry.path().to_string_lossy()));
        }
    }

    result
}
