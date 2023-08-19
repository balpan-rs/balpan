use std::env;

use std::fs::File;

use git2::Repository;
use ignore::WalkBuilder;
use strsim::levenshtein;

pub fn get_current_repository() -> Option<Repository> {
    let current_dir = env::current_dir().ok()?;
    let repo = Repository::discover(current_dir).ok()?;

    Some(repo)
}

pub async fn list_available_files(repo_path: &str) -> Vec<String> {
    let mut result = Vec::new();
    // TODO refactor this
    let walker = WalkBuilder::new(repo_path)
        .hidden(true)
        .git_ignore(true)
        .parents(false)
        .filter_entry(|f| !f.path().to_string_lossy().starts_with("."))
        .filter_entry(|f| !f.path().to_string_lossy().ends_with(".toml"))
        .build();

    for entry in walker.flatten() {
        match entry.file_type() {
            Some(file_type) if file_type.is_file() => {
                if let Ok(_file) = File::open(entry.path()) {
                    result.push(String::from(entry.path().to_string_lossy()));
                }
            }
            // if file type is directory or other things, just skip it
            _ => {
                format!("Skip: {} is not a file", entry.path().to_string_lossy());
                continue;
            }
        }
    }

    result
}

pub fn suggest_subcommand(input: &str) -> Option<String> {
    let dictionary = vec![
        "init", "reset", "grep", "help", "file", "pattern", "format", "json", "plain"
    ];

    let mut closest = None;
    let mut smallest_distance = usize::MAX;

    const THRESHOLD: usize = 3;

    for item in dictionary {
        let distance = levenshtein(input, item);

        match distance {
            0 => return None,
            1..=THRESHOLD if distance < smallest_distance => {
                smallest_distance = distance;
                closest = Some(item.to_string());
            }
            _ => {}
        }
    }

    closest
}