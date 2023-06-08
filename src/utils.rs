use std::env;

use std::fs::File;
use std::io;

use git2::Repository;
use ignore::WalkBuilder;

pub fn get_git_repo_root() -> Option<String> {
    let current_dir = env::current_dir().ok()?;

    let repo = Repository::discover(current_dir).ok()?;
    let repo_root = repo.workdir()?.to_string_lossy().to_string();

    Some(repo_root)
}

pub fn list_available_files(repo_path: &str) -> io::Result<()> {
    let walker = WalkBuilder::new(repo_path)
        .hidden(true)
        .git_ignore(true)
        .parents(false)
        .build();

    // Traverse the directory with gitignore rules applied
    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };

        // Skip directories
        if entry.file_type().expect(".").is_dir() {
            continue;
        }

        // Open each file and process it
        if let Ok(_file) = File::open(entry.path()) {
            println!("File: {:?}", entry.path());
            // Read the file line by line
            // let reader = io::BufReader::new(file);
            // for line in reader.lines() {
            //     if let Ok(line) = line {
            //         // Process each line as needed
            //         println!("{}", line);
            //     }
            // }
        }
    }

    Ok(())
}
