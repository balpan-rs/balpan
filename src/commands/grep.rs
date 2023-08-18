use std::io::BufRead;
use std::{fs::File, io, path::Path};

use serde::{Deserialize, Serialize};

use crate::pattern_search::PatternTree;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GrepReport {
    pub directories: Vec<GrepFile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Directory {
    name: String,
    files: Vec<GrepFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrepFile {
    name: String,
    items: Vec<GrepLine>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GrepLine {
    line: usize,
    content: String,
    position: Vec<usize>,
}

impl GrepReport {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn process_line(
        &mut self,
        line: String,
        index: usize,
        path: &Path,
        pattern_tree: &mut PatternTree,
        patterns: &Vec<String>,
    ) {
        let (found, positions) = pattern_tree.aho_corasick_search(&line, patterns);

        if found {
            // search file in list of files
            let file_name = path.display().to_string();
            let file_index = self.directories.iter().position(|f| f.name == file_name);

            // if file not found, create new file
            if file_index.is_none() {
                self.directories.push(GrepFile {
                    name: file_name.clone(),
                    items: vec![],
                });
            }

            // add line to file
            let file = self
                .directories
                .iter_mut()
                .find(|f| f.name == file_name)
                .unwrap();

            file.items.push(GrepLine {
                line: index,
                content: line.trim_start().to_string(),
                position: positions,
            });
        }
    }

    pub fn grep_file(
        &mut self,
        path: &Path,
        pattern_tree: &mut PatternTree,
        patterns: &Vec<String>,
    ) -> io::Result<()> {
        if let Ok(file) = File::open(path) {
            let r = io::BufReader::new(file);

            for (i, line) in r.lines().enumerate() {
                self.process_line(line.unwrap(), i, path, pattern_tree, patterns);
            }
        }

        Ok(())
    }

    pub fn format_plain(&self) -> String {
        self.directories.iter()
            .flat_map(|directory| {
                directory.items.iter().flat_map(move |file| {
                    file.position.iter().map(move |position| {
                        format!("{}:{}:{}:{}\n", directory.name, file.line, position, file.content)
                    })
                })
            })
            .collect()
    }

    pub fn format_tree(&self, ident_size: usize) -> String {
        let mut result = String::new();
        let whitespace = " ";

        for directory in &self.directories {
            result.push_str(&format!("Directory: {}\n", directory.name));

            for file in directory.name.lines() {
                result.push_str(&format!(
                    "{}File: {}\n",
                    whitespace.repeat(ident_size),
                    file
                ));

                for item in &directory.items {
                    result.push_str(&format!(
                        "{}Line: {}\n",
                        whitespace.repeat(ident_size + 2),
                        item.line
                    ));
                    result.push_str(&format!(
                        "{}Content: {}\n",
                        whitespace.repeat(ident_size + 4),
                        item.content
                    ));
                    result.push_str(&format!(
                        "{}Position: {:?}\n",
                        whitespace.repeat(ident_size + 4),
                        item.position
                    ));
                }
            }
        }

        result
    }
}
