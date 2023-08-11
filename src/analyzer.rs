use std::cell::RefCell;
use std::collections::VecDeque;

use tree_sitter::{Node, Parser, Point, Range, Tree};

use crate::grammar::get_language;
use crate::tree_sitter_extended::{MembershipCheck, RangeFactory};

/// Language enum for `Analyzer` struct
pub enum Language {
    Rust,
    Python,
    Other,
}

impl Language {
    fn as_str(&self) -> &str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            _ => "",
        }
    }
}

impl From<&str> for Language{
    fn from(language_name: &str) -> Self {
        match language_name {
            "rust" => Self::Rust,
            "python" => Self::Python,
            _ => Self::Other,
        }
    }
}

pub struct Analyzer {
    pub source_code: String,
    pub language: Language,
}

pub trait Traversable<'tree> {
    fn get_indent_comment_pool(&self) -> Vec<String>;
    fn get_annotation_whitelist(&self) -> Vec<&str>;
    fn analyze(&self) -> VecDeque<String>;
    fn get_syntax_tree(&self) -> Tree;
    fn get_nested_traversable_symbols(&self) -> Vec<&str>;
    fn get_whitelist_nodes(&self, tree: &'tree Tree) -> Vec<Node<'tree>>;
    fn decorator_node_type(&self) -> &str;
    fn top_level_node_type(&self) -> &str;
}

impl<'tree> Traversable<'tree> for Analyzer {
    fn top_level_node_type(&self) -> &str {
        match self.language {
            Language::Rust => "source_file",
            Language::Python => "module",
            _ => "",
        }
    }

    fn decorator_node_type(&self) -> &str {
        match self.language {
            Language::Rust => "attribute_item",
            Language::Python => "null",
            _ => "",
        }
    }

    fn get_annotation_whitelist(&self) -> Vec<&str> {
        match self.language {
            Language::Rust => vec![
                "attribute_item",
                "mod_item",
                "enum_item",
                "type_item",
                "impl_item",
                "function_item",
                "struct_item",
                "trait_item",
                "macro_definition",
            ],
            Language::Python => vec![
                "class_definition",
                "function_definition",
                "decorated_definition",
            ],
            _ => vec![],
        }
    }

    fn get_indent_comment_pool(&self) -> Vec<String> {
        let comment = match self.language {
            Language::Rust => "/// [TODO]",
            Language::Python => "# [TODO]",
            _ => "//",
        };
        let ident = "    ";
        let max_ident_level = 8;

        (0..max_ident_level)
            .map(|level| {
                let indent = ident.repeat(level);
                format!("{}{}", indent, comment)
            })
            .collect()
    }

    fn get_nested_traversable_symbols(&self) -> Vec<&str> {
        match self.language {
            Language::Rust => vec!["mod_item", "impl_item"],
            Language::Python => vec!["class_definition"],
            _ => vec![],
        }

    }

    fn get_syntax_tree(&self) -> Tree {
        let parser = RefCell::new(Parser::new());
        let language = get_language(self.language.as_str()).unwrap();

        let mut ts_parser = parser.borrow_mut();
        ts_parser
            .set_language(language)
            .expect("treesitter parser for given language does not exists");

        let tree = ts_parser.parse(&self.source_code, None);

        tree.expect("Failed to parsing given source code")
    }

    fn analyze(&self) -> VecDeque<String> {
        let tree = self.get_syntax_tree();
        let nodes = self.get_whitelist_nodes(&tree);

        let nested_traversable_symbols = self.get_nested_traversable_symbols();

        let mut writer_queue = VecDeque::new();
        let mut pending_queue = VecDeque::new();
        let mut nodes_queue = VecDeque::from(nodes);
        let mut indentation_context = VecDeque::new();
        let indent_comment_pool = self.get_indent_comment_pool();

        for (i, line) in self.source_code.lines().enumerate() {
            let row = i;
            let column = line.len();

            let cursor_position = Point { row, column };

            if nodes_queue.is_empty() {
                writer_queue.push_back(line.to_owned());
                continue;
            }

            let current_node = match nodes_queue.front() {
                Some(node) => node,
                None => panic!("Failed to retrieve treesitter node from queue"),
            };

            let indent_size = indentation_context.len();
            let comment_line: String = indent_comment_pool[indent_size].clone();

            let mut pop_node = false;

            match Range::from_node(*current_node) {
                node_range if cursor_position.is_member_of(node_range) => {
                    let node_type = current_node.kind();
                    if node_type == self.decorator_node_type() {
                        pending_queue.push_back(line);
                    } else {
                        writer_queue.push_back(comment_line);
                        if !pending_queue.is_empty() {
                            while !pending_queue.is_empty() {
                                if let Some(queued_line) = pending_queue.pop_front() {
                                    writer_queue.push_back(queued_line.to_owned());
                                }
                            }
                        }
                        writer_queue.push_back(line.to_owned());
                        pop_node = true;
                    }

                    if nested_traversable_symbols.contains(&node_type) {
                        indentation_context.push_back(*current_node);
                        pop_node = true;
                    }

                    if !indentation_context.is_empty() {
                        if let Some(current_context) = indentation_context.front() {
                            if cursor_position == current_context.end_position() {
                                indentation_context.pop_front();
                            }
                        }
                    }

                    if cursor_position == current_node.end_position() {
                        pop_node = true;
                    }

                    if pop_node {
                        nodes_queue.pop_front();
                    }
                }
                _ => {
                    writer_queue.push_back(line.to_owned());
                }
            }
        }

        writer_queue.to_owned()
    }

    /// This methods collects treesitter nodes with BFS
    ///
    /// All of tree sitter nodes are ordered by non decreasing order
    fn get_whitelist_nodes(&self, tree: &'tree Tree) -> Vec<Node<'tree>> {
        let mut deq = VecDeque::new();
        let whitelist = self.get_annotation_whitelist();
        let nested_traversable_symbols = self.get_nested_traversable_symbols();
        let mut result = Vec::new();
        deq.push_back(tree.root_node());

        while !deq.is_empty() {
            if let Some(node) = deq.pop_front() {
                let node_type = node.kind();

                if whitelist.contains(&node_type) {
                    result.push(node);
                }

                if !nested_traversable_symbols.contains(&node_type)
                    && node_type != self.top_level_node_type()
                {
                    continue;
                }

                let mut cursor = node.walk();

                for child_node in node.children(&mut cursor) {
                    deq.push_back(child_node);
                }

                cursor.reset(node);

                if let Some(body) = node.child_by_field_name("body") {
                    let mut body_cursor = body.walk();
                    for child_node in body.children(&mut body_cursor) {
                        deq.push_back(child_node);
                    }
                }
            }
        }

        result.sort_by(|u, v| u.start_position().row.cmp(&v.start_position().row));

        for node in result.iter() {
            println!("{}", node.kind());
        }

        result.to_owned()
    }
}
