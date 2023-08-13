use std::path::Path;

use clap::Command;

use balpan::utils::get_current_repository;
use balpan::scanner::Scanner;
use git2::Repository;

fn git(args: Vec<String>) {
    std::process::Command::new("git")
        .args(args)
        .output();
}

fn find_branch<'a, 'b>(repository: &'b Repository,target: &'a str) -> &'a str {
    let mut iter = repository.branches(None);

    loop {
        if let Some(Ok((branch, _))) = &iter.as_mut().expect("???").next() {
            if let Ok(Some(branch_name)) = branch.name() {
                if target == branch_name {
                    return &target;
                }
            }
        } else {
            break;
        }
    }   

    return "";
}

fn main() {
    let matches = 
        Command::new("balpan")
            .version("0.1.0")
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
    
    let mut is_already_setup = false;

    if let Some(_) = matches.subcommand_matches("init") {
        if let Some(repo) = get_current_repository() {
            let onboarding_branch = find_branch(&repo, "onboarding").to_string();
            is_already_setup = !onboarding_branch.is_empty();

            if main_branch.is_empty() {
                main_branch = find_branch(&repo, "main").to_string();
            }
            
            if main_branch.is_empty() {
                main_branch = find_branch(&repo, "master").to_string();
            }

            if !is_already_setup {
                git(vec!["switch".to_string(), main_branch.clone()]);
                git(vec!["switch".to_string(), "-c".to_string(), onboarding_branch.clone()])
            }

            git(vec!["switch".to_string(), main_branch]);
            git(vec!["switch".to_string(), onboarding_branch]);

            Scanner::scan(&repo);
        }
        println!("init!");
        return
    }

    if let Some(_) = matches.subcommand_matches("reset") {
        if let Some(repo) = get_current_repository() {
            let onboarding_branch = find_branch(&repo, "onboarding").to_string();
            is_already_setup = !onboarding_branch.is_empty();

            if main_branch.is_empty() {
                main_branch = find_branch(&repo, "main").to_string();
            }
            
            if main_branch.is_empty() {
                main_branch = find_branch(&repo, "master").to_string();
            }

            if !is_already_setup {
                git(vec!["switch".to_string(), main_branch.clone()]);
                git(vec!["switch".to_string(), "-c".to_string(), onboarding_branch.clone()])
            }
        }
        return
    }
}

