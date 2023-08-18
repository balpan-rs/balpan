use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use balpan::pattern_search::PatternTree;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use balpan::scanner::Scanner;
use balpan::utils::get_current_repository;
use git2::Repository;
use strsim::levenshtein;

#[derive(Debug, Parser)]
#[command(author, about, version, long_about = None)]
struct BalpanApp {
    #[clap(subcommand)]
    command: BalpanCommand,
}

#[derive(Debug, Subcommand)]
enum BalpanCommand {
    #[clap(about = "Setup environment for Balpan and fetch all available treesitter parsers")]
    Init,
    #[clap(about = "Reset environment for Balpan and removes all TODO comments")]
    Reset,
    #[clap(about = "Search for TODO comments in the current repository")]
    Grep {
        #[clap(short, long, help = "Specific file to scan")]
        file: Option<String>,
        #[clap(short, long, help = "Specific pattern to search")]
        pattern: Option<Vec<String>>,
        #[clap(long, help = "Apply formatting to the output. Available options: json, plain (default)")]
        format: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct GrepItem {
    file: String,
    line: usize,
    position: usize,
    content: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct GrepReport {
    items: Vec<GrepItem>,
}

impl GrepReport {
    fn new() -> Self {
        Default::default()
    }
}

fn main() {
    let app = BalpanApp::parse();

    // verify that the subcommand entered is correct.
    let user_input = std::env::args().nth(1);

    if let Some(input) = user_input {
        if suggest_subcommand(&input).is_some() {
            println!("Did you mean '{}'?", suggest_subcommand(&input).unwrap());
        }
    }

    match app.command {
        BalpanCommand::Init => handle_init(),
        BalpanCommand::Reset => handle_reset(),
        BalpanCommand::Grep {
            file,
            pattern,
            format,
        } => handle_grep(file, pattern, &mut GrepReport::new(), format),
    }
}

fn git(args: Vec<String>) {
    std::process::Command::new("git")
        .args(args)
        .output()
        .unwrap();
}

fn find_branch<'a>(repository: &Repository, target: &'a str) -> Option<&'a str> {
    let mut iter = repository.branches(None);

    while let Some(Ok((branch, _))) = &iter.as_mut().expect("???").next() {
        if let Ok(Some(branch_name)) = branch.name() {
            if target == branch_name {
                return Some(target);
            }
        }
    }

    None
}

fn find_main_or_master_branch<'a>(repo: &'a Repository, branches: &[&'a str]) -> String {
    if branches.is_empty() {
        panic!("No main or master branch found");
    }

    if let Some(branch) = find_branch(repo, branches[0]) {
        return branch.to_string();
    }

    find_main_or_master_branch(repo, &branches[1..])
}

fn suggest_subcommand(input: &str) -> Option<&'static str> {
    let dictionary = vec!["init", "reset", "grep", "help", "file", "pattern", "format"];

    let mut closest = None;
    let mut smallest_distance = usize::MAX;

    const THRESHOLD: usize = 3;

    for item in dictionary {
        let distance = levenshtein(input, item);

        match distance {
            0 => return None,
            1..=THRESHOLD if distance < smallest_distance => {
                smallest_distance = distance;
                closest = Some(item);
            }
            _ => {}
        }
    }

    closest
}

fn handle_reset() {
    let repo = get_current_repository().unwrap();
    //let onboarding_branch = find_branch(&repo, "onboarding").to_string();
    let is_already_setup: bool;

    let onboarding_branch = match find_branch(&repo, "onboarding") {
        Some(branch) => {
            is_already_setup = true;
            branch.to_string()
        }
        None => panic!("No onboarding branch found"),
    };

    let main_branch = find_main_or_master_branch(&repo, &["main", "master"]);

    if is_already_setup {
        git(vec!["switch".to_owned(), main_branch]);
        git(vec![
            "branch".to_owned(),
            "-d".to_owned(),
            onboarding_branch,
        ]);
    }
}

fn handle_init() {
    let repo = get_current_repository().unwrap();
    // let onboarding_branch = find_branch(&repo, "onboarding").to_owned();
    let mut is_already_setup: bool = false;

    let _onboarding_branch = match find_branch(&repo, "onboarding") {
        Some(branch) => {
            is_already_setup = true;
            branch.to_string()
        }
        None => String::new(),
    };

    let main_branch = find_main_or_master_branch(&repo, &["main", "master"]);

    if !is_already_setup {
        git(vec!["switch".to_owned(), main_branch.clone()]);
        git(vec![
            "switch".to_owned(),
            "-c".to_owned(),
            "onboarding".to_owned(),
        ]);
    }

    git(vec!["switch".to_owned(), main_branch]);
    git(vec!["switch".to_owned(), "onboarding".to_owned()]);

    Scanner::scan(&repo);
    println!("init!");
}

fn handle_grep(
    file: Option<String>,
    pattern: Option<Vec<String>>,
    report: &mut GrepReport,
    format: Option<String>,
) {
    let mut pattern_tree = PatternTree::new();
    let default_patterns = vec!["[TODO]".to_string(), "[DONE]".to_string()];
    let patterns_to_search = pattern.unwrap_or(default_patterns);

    if let Some(file_path) = file {
        grep_file(
            Path::new(&file_path),
            report,
            &mut pattern_tree,
            &patterns_to_search,
        )
        .unwrap();
    } else {
        // Scanning all files in the repository
        let repo = get_current_repository().unwrap();
        let path = repo.workdir().expect("Failed to load work directory");
        let mut callback = |p: &Path| grep_file(p, report, &mut pattern_tree, &patterns_to_search);

        visit_dirs(path, &mut callback).unwrap();
    }

    let report = report_formatting(report, format);
    println!("{}", report);
}

fn grep_file(
    path: &Path,
    report: &mut GrepReport,
    pattern_tree: &mut PatternTree,
    patterns: &Vec<String>,
) -> io::Result<()> {
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                process_line(line, index, path, pattern_tree, patterns, report);
            }
        }
    }

    Ok(())
}

fn process_line(
    line: String,
    index: usize,
    path: &Path,
    pattern_tree: &mut PatternTree,
    patterns: &Vec<String>,
    report: &mut GrepReport,
) {
    let (found, matched_positions) = pattern_tree.aho_corasick_search(&line, patterns);

    if found {
        for pos in matched_positions {
            report.items.push(GrepItem {
                file: path.display().to_string(),
                line: index + 1,
                position: pos,
                content: line.clone(),
            });
        }
    }
}

fn visit_dirs(
    dir: &Path,
    callback: &mut dyn for<'a> FnMut(&'a Path) -> io::Result<()>,
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            match path.is_dir() {
                true => visit_dirs(&path, callback)?,
                false => callback(&path)?,
            }
        }
    }

    Ok(())
}

fn report_formatting(report: &mut GrepReport, format: Option<String>) -> String {
    let default = "plain".to_string();
    let format = format.unwrap_or(default);

    match format.as_str() {
        "json" => return serde_json::to_string_pretty(&report).unwrap(),
        "plain" | _ => {
            let mut result = String::new();

            for item in &report.items {
                result.push_str(&format!(
                    "{}:{}:{}:{}\n",
                    item.file, item.line, item.position, item.content
                ));
            }

            return result;
        }
    }
}

#[cfg(test)]
mod main_tests {
    static RUST_EXAMPLE_1: &str = r#"
    /// [TODO] RangeFactory
    pub trait RangeFactory {
        fn from_node(node: Node) -> Range;
    }

    /// [TODO] RangeFactory
    impl RangeFactory for Range {
        /// [TODO] RangeFactory > from_node
        #[inline]
        fn from_node(node: Node) -> Range {
            Range {
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
                start_point: node.start_position(),
                end_point: node.end_position(),
            }
        }
    }"#;

    static RUST_EXAMPLE_2: &str = r#"
    /// [TODO] tree_sitter_extended
    mod tree_sitter_extended {
        /// [DONE] tree_sitter_extended > RangeFactory
        pub trait RangeFactory {
            fn from_node(node: Node) -> Range;
        }

        /// [TODO] tree_sitter_extended > RangeFactory
        impl RangeFactory for Range {
            /// [DONE] tree_sitter_extended > RangeFactory > from_node
            #[inline]
            fn from_node(node: Node) -> Range {
                Range {
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    start_point: node.start_position(),
                    end_point: node.end_position(),
                }
            }
        }
    }"#;
    
    #[test]
    fn grep_python_command() {
        use super::*;
        use tempfile::tempdir;

        // Crete temporary directory and rust code file
        let dir = tempdir().unwrap();
        let python_file = dir.path().join("dummy.py");
        std::fs::write(
            &python_file,
            r#"
            # [TODO] some
            def some(a, b):
                return a + b

            # [TODO] class
            class DummyPythonClass:
                # [DONE] __init__
                def __init__(self):
                    print("hello")
                
                # [TODO] foo
                def foo(self, a, b):
                    return a + b"#,
        )
        .unwrap();

        let mut report = GrepReport { items: Vec::new() };

        let mut pattern_tree = PatternTree::new();
        let patterns = vec!["[TODO]".to_string(), "[DONE]".to_string()];

        grep_file(&python_file, &mut report, &mut pattern_tree, &patterns).unwrap();

        assert_eq!(report.items.len(), 4);

        let report = serde_json::to_string_pretty(&report).unwrap();
        println!("{}", report);
    }

    #[test]
    fn grep_scan_specific_file() {
        use super::*;
        use serde_json::to_string_pretty;
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let rust_file1 = dir.path().join("dummy1.rs");
        let rust_file2 = dir.path().join("dummy2.rs");

        let mut file = File::create(&rust_file1).unwrap();
        file.write_all(RUST_EXAMPLE_1.as_bytes()).unwrap();

        let mut file = File::create(&rust_file2).unwrap();
        file.write_all(RUST_EXAMPLE_2.as_bytes()).unwrap();

        let mut report = GrepReport { items: Vec::new() };

        let pattern = vec!["[TODO]".to_string()];

        // cargo run -- grep -f ../dummy1.rs
        grep_file(&rust_file1, &mut report, &mut PatternTree::new(), &pattern).unwrap();

        let report_info = to_string_pretty(&report).unwrap();
        println!("{}", report_info);

        // cargo run -- grep -f ../dummy2.rs
        let mut report = GrepReport { items: Vec::new() };

        let pattern = vec!["[TODO]".to_string(), "[DONE]".to_string()];

        grep_file(&rust_file2, &mut report, &mut PatternTree::new(), &pattern).unwrap();

        let report_info = to_string_pretty(&report).unwrap();
        println!("{}", report_info);
    }

    #[test]
    fn test_hadle_grep() {
        use super::*;
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let rust_file = dir.path().join("dummy1.rs");
        
        let mut file = File::create(&rust_file).unwrap();
        file.write_all(RUST_EXAMPLE_2.as_bytes()).unwrap();

        let mut report = GrepReport { items: Vec::new() };
        let pattern = vec!["[TODO]".to_string()];
        
        let _format = Some("plain".to_string());
        handle_grep(Some(rust_file.to_str().unwrap().to_string()), None, &mut report, None);

        let mut report = GrepReport { items: Vec::new() };
        let format = Some("json".to_string());
        handle_grep(Some(rust_file.to_str().unwrap().to_string()), Some(pattern), &mut report, format);
    }
}
