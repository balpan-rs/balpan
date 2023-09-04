use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use git2::Repository;

use crate::analyzer::Analyzer;
use crate::grammar::{build_grammars, fetch_grammars};
use crate::language::Language;
use crate::utils::list_available_files;

pub struct Scanner;

impl Scanner {
    pub async fn scan(repo: &Repository) {
        fetch_grammars().unwrap();
        build_grammars(None).unwrap();

        if let Some(workdir) = repo.workdir() {
            let repo_root = workdir.to_string_lossy();
            let filenames = list_available_files(&repo_root);
            for filename in filenames.await {
                if filename.contains("test") {
                    continue;
                }
                let path = Path::new(&filename);
                let language = match path.extension() {
                    Some(os_str) => Language::from_extension(os_str.to_str().unwrap()),
                    _ => Language::Other("".to_string()),
                };

                if let Language::Other(_) = language {
                    continue;
                }

                if let Ok(mut file) = File::options().read(true).write(true).open(path) {
                    let mut source_code = String::new();
                    file.read_to_string(&mut source_code).unwrap();
                    let with_empty_line = source_code.ends_with('\n');
                    let analyzer = Analyzer {
                        source_code,
                        language,
                    };

                    let writer_queue = &analyzer.analyze();
                    let mut lines = vec![];

                    for line in writer_queue {
                        lines.push(String::from(line));
                    }

                    if with_empty_line {
                        lines.push(String::new());
                    }

                    file.set_len(0).unwrap();
                    file.rewind().unwrap();
                    file.write_all(lines.join("\n").as_bytes()).unwrap();
                }
            }
        }
    }

    /// Scan a specific file and add TODO comments
    pub async fn scan_specific_file(path: PathBuf) {
        fetch_grammars().unwrap();
        build_grammars(None).unwrap();

        if let Ok(mut file) = File::options().read(true).write(true).open(path.clone()) {
            let mut source_code = String::new();
            file.read_to_string(&mut source_code).unwrap();
            let with_empty_line = source_code.ends_with('\n');

            let language = match path.extension() {
                Some(p) => Language::from_extension(p.to_str().unwrap()),
                _ => Language::Other(String::new()),
            };

            let analyzer = Analyzer {
                source_code,
                language,
            };

            let writer_queue = &analyzer.analyze();
            let mut lines: Vec<String> = vec![];

            for line in writer_queue {
                lines.push(String::from(line));
            }

            if with_empty_line {
                lines.push(String::new());
            }

            file.set_len(0).unwrap();
            file.rewind().unwrap();
            file.write_all(lines.join("\n").as_bytes()).unwrap();
        }
    }
}
