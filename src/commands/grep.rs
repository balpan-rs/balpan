use std::path::PathBuf;
use std::{io, path::Path};

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use serde::{Deserialize, Serialize};

use crate::pattern_search::PatternTree;
use crate::utils::suggest_subcommand;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GrepReport {
    pub directories: Vec<Directory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    name: String,
    files: Vec<GrepFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrepFile {
    pub name: String,
    pub items: Vec<GrepLine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrepLine {
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
        //let (found, positions) = pattern_tree.aho_corasick_search(&line, patterns);
        let (found, positions) = pattern_tree.selective_search(patterns, &line);

        if found {
            // search file in list of files
            let dir_name = path.parent().unwrap().display().to_string();
            let file_name = path.display().to_string();
            
            let dir_index = self.directories
                .iter()
                .position(|d| d.name == dir_name);

            if dir_index.is_none() {
                self.directories.push(Directory {
                    name: dir_name.clone(),
                    files: Vec::new(),
                });
            }

            let dir = self.directories
                .iter_mut()
                .find(|d| d.name == dir_name)
                .unwrap();

            let file_index = dir.files
                .iter()
                .position(|f| f.name == file_name);

            if file_index.is_none() {
                dir.files.push(GrepFile {
                    name: file_name.clone(),
                    items: Vec::new(),
                });
            }

            let file = dir
                .files
                .iter_mut()
                .find(|f| f.name == file_name)
                .unwrap();

            let line = GrepLine {
                line: index + 1,
                content: line,
                position: positions,
            };
            file.items.push(line);
        }
    }

    pub async fn grep_file(
        &mut self,
        path: &Path,
        pattern_tree: &mut PatternTree,
        patterns: &Vec<String>,
    ) -> io::Result<()> {
        let file = File::open(path).await?;
        let mut reader = BufReader::new(file);

        let mut line_bytes = Vec::new();
        let mut i = 0;

        while reader.read_until(b'\n', &mut line_bytes).await? > 0 {
            let line = String::from_utf8_lossy(&line_bytes).to_string();
            self.process_line(line, i, path, pattern_tree, patterns);
            line_bytes.clear();
            i += 1;
        }

        Ok(())
    }

    // TODO
    pub fn format_tree(&self, ident_size: usize) -> String {
        let mut result = String::new();

        for directory in &self.directories {
            result.push_str(&format!("{}\n", directory.name));

            for file in &directory.files {
                for item in &file.items {
                    let file_relative_path = GrepReport::display_relative_path(&directory.name, &file.name);

                    result.push_str(&format!(
                        "{:ident$}{}:{}:{} - {}\n",
                        "",
                        file_relative_path,
                        item.line,
                        item.position[0],
                        item.content.trim_start(),
                        ident = ident_size,
                    ));
                }
            }
        }

        result
    }

    fn format_plain(&self) -> String {
        let mut result = String::new();

        for dir in &self.directories {
            let path = Path::new(&dir.name);
            
            // directory path
            let last_two: Vec<&str> = path.iter().rev().take(2).map(|s| s.to_str().unwrap()).collect();

            if last_two.len() == 2 {
                result.push_str(&format!("{}/{}\n", last_two[1], last_two[0]));
            } else {
                result.push_str(&format!("{}\n", last_two[0]));
            }
        
            for file in &dir.files {
                let file_name = Path::new(&file.name);
                let last_two: Vec<&str> = file_name.iter().rev().take(2).map(|s| s.to_str().unwrap()).collect();
                result.push_str(&format!("{}\n", last_two[0]));

                for item in &file.items {
                    result.push_str(&format!("{}    {}", item.line, item.content.trim_start()));
                }

                result.push_str("\n");
            }

            result.push_str("\n");
        }

        result
    }

    pub fn report_formatting(&mut self, format: Option<String>) -> String {
        let default = "plain".to_string();
        let format = format.unwrap_or(default);
    
        match format.as_str() {
            "json" => serde_json::to_string_pretty(self).unwrap(),
            "plain" => self.format_plain(),
            // "tree" => self.format_tree(4),
            _ => {
                let suggest = suggest_subcommand(&format).unwrap();
                format!("Unknown format: '{}'. Did you mean '{}'?", format, suggest)
            }
        }
    }

    fn display_relative_path(directory: &str, file_name: &str) -> String {
        let base_path = Path::new(directory);
        let path = Path::new(file_name);

        let relative_path = path.strip_prefix(base_path).unwrap();
        let mut display_path = PathBuf::new();

        for _ in 1..base_path.components().count() - 2 {
            display_path.push("..");
        }

        display_path.push(relative_path);

        display_path.display().to_string()
    }
}
