use std::path::Path;
use std::time::Instant;

use balpan::commands::pattern_search::PatternTree;
use clap::{Parser, Subcommand};

use balpan::commands::grep::GrepReport;
use balpan::scanner::Scanner;
use balpan::utils::{get_current_repository, list_available_files, suggest_subcommand};
use git2::Repository;
use tokio::runtime::{Builder, Runtime};

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
    #[clap(
        about = "Searches a particular pattern of characters, and displays all lines that contain that pattern"
    )]
    Grep {
        #[clap(short = 'f', long, help = "Specific file to scan")]
        file: Option<String>,
        #[clap(short = 'p', long, help = "Specific pattern to search")]
        pattern: Option<String>,
        #[clap(
            long,
            help = "Apply formatting to the output. Available options: json, tree, plain (default)"
        )]
        #[clap(
            short = 'i',
            long = "ignore",
            help = "ignores the case(upper or lower) of the pattern."
        )]
        ignore_case: Option<Vec<String>>,
        #[clap(
            short = 'H',
            help = "Display the matched lines, but do not display the filenames."
        )]
        hide_path: bool,
        #[clap(
            short = 'l',
            help = "Display the names of files that contain matches, without displaying the matched lines."
        )]
        list_of_files: bool,
        #[clap(
            short = 'c',
            help = "This prints only a count of the lines that match a pattern."
        )]
        count: bool,
        #[clap(
            short = 'T',
            long = "time",
            help = "Display the elapsed time during the execution of the command."
        )]
        show_elapsed_time: bool,
        #[clap(short = 'o', help = "Colorize the matched pattern in the output.")]
        colorize: bool,
        #[clap(
            short = 'E',
            help = "Treats pattern as an extended regular expression (ERE)."
        )]
        extended_regex: bool,
        format: Option<String>,
    },
}

fn create_runtime() -> Runtime {
    Builder::new_current_thread().enable_all().build().unwrap()
}

fn main() {
    let app = BalpanApp::parse();

    // verify that the subcommand entered is correct.
    let user_input: Option<String> = std::env::args().nth(1);

    if let Some(input) = user_input {
        if suggest_subcommand(&input).is_some() {
            println!("Did you mean '{}'?", suggest_subcommand(&input).unwrap());
        }
    }

    match app.command {
        BalpanCommand::Init => {
            let runtime = create_runtime();

            runtime.block_on(async { handle_init().await })
        }
        BalpanCommand::Reset => handle_reset(),
        BalpanCommand::Grep {
            file,
            pattern,
            format,
            ignore_case,
            hide_path,
            list_of_files,
            count,
            colorize,
            extended_regex,
            show_elapsed_time: elapsed,
        } => {
            let time = Instant::now();
            let runtime = create_runtime();

            let patterns: Option<Vec<String>> =
                pattern.map(|p| p.split_whitespace().map(|s| s.to_string()).collect());

            runtime.block_on(async {
                let mut report = GrepReport::new();
                handle_grep(
                    file,
                    patterns,
                    &mut report,
                    format,
                    ignore_case,
                    hide_path,
                    list_of_files,
                    count,
                    colorize,
                    extended_regex,
                )
                .await;
            });

            if elapsed {
                println!("time: {:?}", time.elapsed());
            }
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

    while let Some(Ok((ref branch, _))) = &iter.as_mut().expect("???").next() {
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

#[allow(clippy::too_many_arguments)]
async fn handle_grep(
    file: Option<String>,
    pattern: Option<Vec<String>>,
    report: &mut GrepReport,
    format: Option<String>,
    ignore_case: Option<Vec<String>>,
    hide_path: bool,
    list_of_files: bool,
    count: bool,
    colorize: bool,
    extends_regex: bool,
) {
    let mut pattern_tree = PatternTree::new();
    let default_patterns = vec!["[TODO]".to_string(), "[DONE]".to_string()];

    let patterns_to_search: Vec<String>;

    if extends_regex {
        pattern_tree.ignore_case = true;
        pattern_tree.regex_flag = true;
    }

    match ignore_case {
        Some(ignore_patterns) => {
            pattern_tree.ignore_case = true;
            patterns_to_search = ignore_patterns;
        }
        None => {
            patterns_to_search = pattern.unwrap_or(default_patterns);
        }
    }

    match file {
        Some(file_path) => {
            scan_specific_file(file_path, report, &mut pattern_tree, &patterns_to_search).await
        }
        None => scan_project_directory(report, pattern_tree, patterns_to_search.clone()).await,
    }

    let formatting = report.report_formatting(
        format,
        hide_path,
        list_of_files,
        count,
        patterns_to_search,
        colorize,
    );
    println!("{}", formatting);
}

async fn scan_project_directory(
    report: &mut GrepReport,
    mut pattern_tree: PatternTree,
    patterns_to_search: Vec<String>,
) {
    let repo = get_current_repository().expect("No repository found");
    let repo_path = repo.workdir().expect("No workdir found").to_str().unwrap();

    let available_files: Vec<String> = list_available_files(repo_path).await;

    for file in available_files {
        let path = Path::new(&file);
        update_report(report, path, &mut pattern_tree, &patterns_to_search).await;
    }
}

async fn scan_specific_file(
    file_path: String,
    report: &mut GrepReport,
    pattern_tree: &mut PatternTree,
    patterns_to_search: &Vec<String>,
) {
    let path = Path::new(&file_path);
    update_report(report, path, pattern_tree, patterns_to_search).await;
}

async fn update_report(
    report: &mut GrepReport,
    path: &Path,
    pattern_tree: &mut PatternTree,
    patterns_to_search: &Vec<String>,
) {
    report
        .grep_file(path, pattern_tree, patterns_to_search)
        .await
        .unwrap();
}
