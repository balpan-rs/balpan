use clap::{Parser, Subcommand};

use balpan::scanner::Scanner;
use balpan::utils::get_current_repository;
use git2::Repository;
use strsim::levenshtein;

#[derive(Debug, Parser)]
#[command(author, about, version)]
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
}
