use std::fs::File;
use std::io::{Read, Write, Seek};
use std::path::Path;

use git2::Repository;

use crate::grammar::{fetch_grammars, build_grammars};
use crate::utils::list_available_files;
use crate::analyzer::{Analyzer, Traversable};
use crate::language::Language;


pub struct Scanner;

impl Scanner {
    #[inline]
    pub fn scan(repo: &Repository) {
        fetch_grammars().unwrap();
        build_grammars(None).unwrap();

        if let Some(workdir) = repo.workdir() {
            let repo_root = workdir.to_string_lossy().to_string();
            let filenames = list_available_files(&repo_root);
            for filename in filenames {
                if filename.contains("test") {
                    continue;
                }
                let path = Path::new(&filename);
                let language = match path.extension() {
                    Some(os_str) => Language::from_extension(os_str.to_str().unwrap()),
                    _ => Language::Other("".to_string()),
                };

                match language {
                    Language::Other(_) => continue,
                    _ => {}
                };

                if let Ok(mut file) = File::options().read(true).write(true).open(path) {
                    let mut source_code = String::new();
                    file.read_to_string(&mut source_code);
                    let analyzer = Analyzer {
                        source_code,
                        language: Language::from(language),
                    };

                    let writer_queue = &analyzer.analyze();
                    let mut lines = vec![];

                    for line in writer_queue {
                        lines.push(String::from(line));
                    }

                    file.set_len(0).unwrap();
                    file.rewind().unwrap();
                    file.write_all(lines.join("\n").as_bytes()).unwrap();
                }
            }

        }
    }
}
