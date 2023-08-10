use clap::Command;

use balpan::utils::get_current_repository;

fn git(args: Vec<String>) {
    std::process::Command::new("git")
        .args(args)
        .output();
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
            .get_matches();

    let main_branch = "main".to_string();
    let onboarding_branch = "onboarding".to_string();
    let mut is_already_setup = false;

    if let Some(_) = matches.subcommand_matches("init") {
        if let Some(repo) = get_current_repository() {
            let mut iter = repo.branches(None);
            // let mut iter = branches.iter();

            loop {
                if let Some(Ok((branch, _))) = &iter.as_mut().expect("???").next() {
                    if let Ok(Some(branch_name)) = branch.name() {
                        if "onboarding" == branch_name {
                            is_already_setup = true;
                        }
                    }
                } else {
                    break;
                }
            }   

            if !is_already_setup {
                git(vec!["switch".to_string(), main_branch]);
                git(vec!["switch".to_string(), "-c".to_string(), onboarding_branch])
            }
            
        }
        println!("init!");
    }
}

