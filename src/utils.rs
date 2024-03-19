use std::collections::HashSet;
use std::env;
use std::fs::File;

use git2::Repository;
use ignore::{DirEntry, WalkBuilder};
use once_cell::sync::Lazy;
use strsim::levenshtein;

#[rustfmt::skip]
static IGNORED_EXTENSIONS: Lazy<HashSet<String>> = Lazy::new(|| {
    [
        ".tmp", ".bak", ".swp", ".old", ".new", ".orig", ".patch", ".diff", // temporary
        ".proj", ".sln", ".classpath", ".project",                          // project 
        ".obj", ".exe", ".dll", ".class", ".o", ".e",                       // binary
        ".toml", ".lock", ".json", ".md", ".yaml", ".yml", ".xml", ".ini",  // dev config
        ".zip", ".tar", ".gz", ".rar", ".7z", ".tgz", ".xz", ".bz2",        // compressed
        ".png", ".jpg", ".jpeg", ".bmp", ".svg", ".gif",                    // image
        ".wav", ".mp3", ".mp4", ".avi", ".mov", ".flv", ".ogg",             // audio/video
        ".doc", ".docx", ".pdf", ".ppt", ".pptx", ".xls", "xlsx", ".odt",   // document
        ".yml", ".xml", ".ini",                                             // config
        ".log", ".dat",                                                     // log
        ".yarn", ".npm",                                                    // package manager
    ]
    .iter()
    .map(|&s| s.into())
    .collect()
});

static IGNORED_PREFIXES: Lazy<HashSet<String>> = Lazy::new(|| {
    ["."].iter().map(|&s| s.into()).collect() // hidden files start with '.'
});

pub fn get_current_repository() -> Option<Repository> {
    let current_dir = env::current_dir().ok()?;
    let repo = Repository::discover(current_dir).ok()?;

    Some(repo)
}

pub async fn list_available_files(repo_path: &str) -> Vec<String> {
    let mut result = Vec::new();

    let is_ignored = move |entry: &DirEntry| {
        let extension = entry
            .path()
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let file_name = entry
            .path()
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        IGNORED_EXTENSIONS.contains(&format!(".{}", extension))
            || IGNORED_PREFIXES
                .iter()
                .any(|prefix| file_name.starts_with(prefix))
    };

    let walker = WalkBuilder::new(repo_path)
        .hidden(true)
        .git_ignore(true)
        .parents(false)
        .filter_entry(move |f| !is_ignored(f))
        .build();

    for entry in walker.flatten() {
        match entry.file_type() {
            Some(file_type) if file_type.is_file() => {
                if let Ok(_file) = File::open(entry.path()) {
                    result.push(entry.path().to_string_lossy().to_string());
                }
            }
            // if file type is directory or other things, just skip it
            _ => continue,
        }
    }

    result
}

#[rustfmt::skip]
static DICTIONARY: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "init", "reset", "grep", "help", "file", "pattern", "format", "json", "plain",
    ]
});

pub fn suggest_subcommand(input: &str) -> Option<String> {
    let mut closest = None;
    let mut smallest_distance = 80; // default maximum line length setting for COBOL
    const THRESHOLD: usize = 3;

    for item in &*DICTIONARY {
        let distance = levenshtein(input, *item);
        match distance {
            0 => return None,
            1..=THRESHOLD if distance < smallest_distance => {
                smallest_distance = distance;
                closest = Some((*item).to_string());
            }
            _ => {}
        }
    }

    closest
}