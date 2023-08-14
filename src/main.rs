use clap::Command;

use balpan::scanner::Scanner;
use balpan::utils::get_current_repository;
use git2::Repository;
use strsim::levenshtein;

fn git(args: Vec<String>) {
    std::process::Command::new("git")
        .args(args)
        .output()
        .unwrap();
}

fn find_branch<'a>(repository: &Repository, target: &'a str) -> &'a str {
    let mut iter = repository.branches(None);

    while let Some(Ok((branch, _))) = &iter.as_mut().expect("???").next() {
        if let Ok(Some(branch_name)) = branch.name() {
            if target == branch_name {
                return target;
            }
        }
    }

    ""
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

fn main() {
    let matches = Command::new("balpan")
            .version("0.2.0")
            .author("Jaeyeol Lee <rijgndqw012@gmail.com>")
            .about("Balpan CLI automatically generates TODO comments above definition of function/method/class/module and so on.")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("init")
                    .about("Setup environment for Balpan and fetch all available treesitter parsers")
            )
            .subcommand(
                Command::new("reset")
                    .about("Reset environment for Balpan and removes all TODO comments")
            )
            .get_matches();

    let mut main_branch = String::new();

    let is_already_setup;

    // verify that the subcommand entered is correct.
    if let Some(command) = matches.subcommand_name() {
        if suggest_subcommand(command).is_some() {
            eprintln!("Did you mean '{}'?", suggest_subcommand(command).unwrap());
            return;
        }

        eprintln!("`{}` is an unknown command. Enter `help` to check the list of available commands.", command);
    }

    if matches.subcommand_matches("init").is_some() {
        let repo = get_current_repository().unwrap();
        let onboarding_branch = find_branch(&repo, "onboarding").to_owned();
        is_already_setup = !onboarding_branch.is_empty();

        if main_branch.is_empty() {
            main_branch = find_branch(&repo, "main").to_owned();
        }

        if main_branch.is_empty() {
            main_branch = find_branch(&repo, "master").to_owned();
        }

        if !is_already_setup {
            git(vec!["switch".to_owned(), main_branch.clone()]);
            git(vec![
                "switch".to_owned(),
                "-c".to_owned(),
                "onboarding".to_owned(),
            ])
        }

        git(vec!["switch".to_owned(), main_branch]);
        git(vec!["switch".to_owned(), "onboarding".to_owned()]);

        Scanner::scan(&repo);
        println!("init!");
        return;
    }

    if matches.subcommand_matches("reset").is_some() {
        let repo = get_current_repository().unwrap();
        let onboarding_branch = find_branch(&repo, "onboarding").to_owned();
        is_already_setup = !onboarding_branch.is_empty();

        if main_branch.is_empty() {
            main_branch = find_branch(&repo, "main").to_owned();
        }

        if main_branch.is_empty() {
            main_branch = find_branch(&repo, "master").to_owned();
        }

        if is_already_setup {
            git(vec!["switch".to_owned(), main_branch]);
            git(vec![
                "branch".to_owned(),
                "-d".to_owned(),
                onboarding_branch,
            ])
        }
    }
}


#[cfg(test)]
mod main_tests {

    #[test]
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