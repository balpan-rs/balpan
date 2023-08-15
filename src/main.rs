use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

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
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct GrepItem {
    file: String,
    line: usize,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GrepReport {
    items: Vec<GrepItem>,
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
        BalpanCommand::Grep { file } => handle_grep(file, &mut GrepReport { items: Vec::new() }),
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
    let subcommands = vec!["init", "reset"];

    let mut closest = None;
    let mut smallest_distance = usize::MAX;

    const THRESHOLD: usize = 3;

    for subcommand in subcommands {
        let distance = levenshtein(input, subcommand);

        match distance {
            0 => return None,
            1..=THRESHOLD if distance < smallest_distance => {
                smallest_distance = distance;
                closest = Some(subcommand);
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
        },
        None => panic!("No onboarding branch found"),
    };

    let main_branch = find_main_or_master_branch(&repo, &["main", "master"]);

    if is_already_setup {
        git(vec!["switch".to_owned(), main_branch]);
        git(vec!["branch".to_owned(), "-d".to_owned(), onboarding_branch]);
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
        },
        None => String::new(),
    };

    let main_branch = find_main_or_master_branch(&repo, &["main", "master"]);

    if !is_already_setup {
        git(vec!["switch".to_owned(), main_branch.clone()]);
        git(vec!["switch".to_owned(), "-c".to_owned(), "onboarding".to_owned()]);
    }

    git(vec!["switch".to_owned(), main_branch]);
    git(vec!["switch".to_owned(), "onboarding".to_owned()]);

    Scanner::scan(&repo);
    println!("init!");
}

fn handle_grep(file: Option<String>, report: &mut GrepReport) {
    if let Some(file_path) = file {
        // Scanning specific file
        grep_file(Path::new(&file_path), report).unwrap();
    } else {
        // Scanning all files in the repository
        let repo = get_current_repository().unwrap();
        let path = repo.workdir().expect("Failed to load work directory");
        let mut callback = |p: &Path| grep_file(p, report);
        visit_dirs(&path, &mut callback).unwrap();
    }
}

fn grep_file(path: &Path, report: &mut GrepReport) -> io::Result<()> {
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                if line.contains("[TODO]") || line.contains("[DONE]") {
                    report.items.push(GrepItem {
                        file: path.display().to_string(),
                        line: index + 1,
                        content: line,
                    });
                }
            }
        }
    }

    Ok(())
}

fn visit_dirs(dir: &Path, callback: &mut dyn for<'a> FnMut(&'a Path) -> io::Result<()>) -> io::Result<()> {
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

#[cfg(test)]
mod main_tests {
    #[test]
    #[ignore]
    fn subcommand_suggestion() {
        use super::suggest_subcommand;

        assert_eq!(suggest_subcommand("innit"), Some("init"));
        assert_eq!(suggest_subcommand("resett"), Some("reset"));
        assert_eq!(suggest_subcommand("unknown"), None);
        assert_eq!(suggest_subcommand("inot"), Some("init"));

        assert_eq!(suggest_subcommand("init"), None);
        assert_eq!(suggest_subcommand("reset"), None);
    }

    #[test]
    #[ignore]
    fn grep_rust_command() {
        use super::*;
        use tempfile::tempdir;

        // Crete temporary directory and rust code file
        let dir = tempdir().unwrap();

        let rust_file = dir.path().join("dummy.rs");
        std::fs::write(&rust_file, r#"
            /// [TODO]
            fn main() {
                unimplemented!();
            }"#).unwrap();

        let mut report = GrepReport {
            items: Vec::new(),
        };

        grep_file(&rust_file, &mut report).unwrap();

        assert_eq!(report.items.len(), 1);

        let item = &report.items[0];
        assert_eq!(item.file, rust_file.display().to_string());

        assert_eq!(item.line, 2);
        assert_eq!(item.content, "            /// [TODO]");

        let report_info = serde_json::to_string(&report).unwrap();
        println!("{}", report_info);
    }

    #[test]
    #[ignore]
    fn grep_python_command() {
        use super::*;
        use tempfile::tempdir;

        // Crete temporary directory and rust code file
        let dir = tempdir().unwrap();
        let python_file = dir.path().join("dummy.py");
        std::fs::write(&python_file, r#"
            # [TODO]
            class DummyPythonClass:
                # [TODO]
                def __init__(self):
                    print("hello")
                
                # [TODO]
                def foo(self, a, b):
                    return a + b"#).unwrap();

        let mut report = GrepReport {
            items: Vec::new(),
        };

        grep_file(&python_file, &mut report).unwrap();

        assert_eq!(report.items.len(), 3);

        let item = &report.items[0];
        assert_eq!(item.file, python_file.display().to_string());

        let report_info = serde_json::to_string(&report).unwrap();
        println!("{}", report_info);
    }

    #[test]
    #[ignore]
    fn grep_scan_specific_file() {
        use super::*;
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;
        use serde_json::to_string_pretty;

        let dir = tempdir().unwrap();
        let rust_file1 = dir.path().join("dummy1.rs");
        let rust_file2 = dir.path().join("dummy2.rs");

        {
            let mut file = File::create(&rust_file1).unwrap();
            file.write_all(r#"
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
            }"#.as_bytes()).unwrap();
        }

        {
            let mut file = File::create(&rust_file2).unwrap();
            file.write_all(r#"
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
            }"#.as_bytes()).unwrap();

            let mut report = GrepReport {
                items: Vec::new(),
            };

            // cargo run -- grep -f ../dummy1.rs
            handle_grep(Some(rust_file1.display().to_string()), &mut report);

            let report_info = to_string_pretty(&report).unwrap();
            println!("{}", report_info);

            // cargo run -- grep -f ../dummy2.rs
            let mut report = GrepReport {
                items: Vec::new(),
            };

            handle_grep(Some(rust_file2.display().to_string()), &mut report);

            let report_info = to_string_pretty(&report).unwrap();
            println!("{}", report_info);
        }
    }

    #[test]
    #[ignore]
    fn grep_scan_all_files() {
        use crate::{GrepReport, handle_grep};
        use serde_json::to_string_pretty;

        let mut report = GrepReport {
            items: Vec::new(),
        };

        // cargo run -- grep
        handle_grep(None, &mut report);

        let report_info = to_string_pretty(&report).unwrap();
        println!("{}", report_info);
    }
}