use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

use balpan::pattern_search::PatternTree;
use clap::{Parser, Subcommand};

use balpan::commands::grep::GrepReport;
use balpan::scanner::Scanner;
use balpan::utils::{get_current_repository, list_available_files, suggest_subcommand};
use git2::Repository;
use tokio::runtime::{Runtime, Builder};

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

fn create_runtime() -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
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
        BalpanCommand::Init => {
            let runtime = create_runtime();

            runtime.block_on(async {
                handle_init().await
            })
        },
        BalpanCommand::Reset => handle_reset(),
        BalpanCommand::Grep {
            file,
            pattern,
            format,
        } => {
            let time = Instant::now();
            let runtime = create_runtime();
            
            runtime.block_on(async {
                let mut report = GrepReport::new();
                handle_grep(file, pattern, &mut report, format).await;
            });
            println!("time: {:?}", time.elapsed());
        }
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

async fn handle_init() {
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

    Scanner::scan(&repo).await;
    println!("init!");
}

async fn handle_grep(
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
        update_report(report, path, &mut pattern_tree, &patterns_to_search).await;
    } else {
        // Scanning all files in the repository
        let repo = get_current_repository().expect("No repository found");
        let path = repo.workdir().expect("No workdir found").to_str().unwrap();
        
        let available_files = list_available_files(&path).await;

        for file in available_files {
            let path = Path::new(&file);
            update_report(report, path, &mut pattern_tree, &patterns_to_search).await;
        }
    }

    let formatting = report.report_formatting(format);
    println!("{}", formatting);
}

async fn update_report(report: &mut GrepReport, path: &Path, pattern_tree: &mut PatternTree, patterns_to_search: &Vec<String>) {
    report
        .grep_file(path, pattern_tree, patterns_to_search)
        .await
        .unwrap();
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

    #[tokio::test]
    #[ignore]
    async fn grep_python_command() {
        use crate::{handle_grep, GrepReport};

        let report = &mut GrepReport {
            directories: Vec::new(),
        };

        let pattern = vec!["[TODO]".to_string()];

        let time = std::time::Instant::now();
        handle_grep(
            None,
            Some(pattern),
            report,
            None,
        ).await;
        println!("Time elapsed: {:?}", time.elapsed());
    }
}