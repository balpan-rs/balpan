use std::path::Path;

use balpan::pattern_search::PatternTree;
use clap::{Parser, Subcommand};

use balpan::commands::grep::GrepReport;
use balpan::scanner::Scanner;
use balpan::utils::{get_current_repository, list_available_files, suggest_subcommand};
use git2::Repository;

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
        #[clap(
            long,
            help = "Apply formatting to the output. Available options: json, tree, plain (default)"
        )]
        format: Option<String>,
    },
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
        let path = Path::new(&file_path);
        report
            .grep_file(path, &mut pattern_tree, &patterns_to_search)
            .unwrap();
    } else {
        // Scanning all files in the repository
        let repo = get_current_repository().expect("No repository found");
        let path = repo.workdir().expect("No workdir found").to_str().unwrap();
        
        let available_files = list_available_files(&path);

        for file in available_files {
            let path = Path::new(&file);
            report
                .grep_file(path, &mut pattern_tree, &patterns_to_search)
                .unwrap();
        }
    }

    let formatting = report.report_formatting(format);
    println!("{}", formatting);
}

#[cfg(test)]
mod main_tests {
    static _RUST_EXAMPLE_1: &str = r#"
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
    #[ignore]
    fn grep_python_command() {
        use crate::{handle_grep, GrepReport};
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

        let report = &mut GrepReport {
            directories: Vec::new(),
        };

        let pattern = vec!["[TODO]".to_string()];

        handle_grep(
            None,
            Some(pattern),
            report,
            None,
        );
    }

    #[test]
    #[ignore]
    fn test_handle_grep() {
        use crate::{handle_grep, GrepReport};
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let rust_file = dir.path().join("dummy1.rs");

        let mut file = File::create(&rust_file).unwrap();
        file.write_all(RUST_EXAMPLE_2.as_bytes()).unwrap();

        let mut report = GrepReport {
            directories: Vec::new(),
        };

        let pattern = vec!["[TODO]".to_string()];
        let format = Some("tree".to_string());

        // balpan grep -f ../dummy1.rs
        handle_grep(
            Some(rust_file.to_str().unwrap().to_string()),
            Some(pattern),
            &mut report,
            format,
        );
    }
}