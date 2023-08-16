use std::cell::RefCell;
use std::collections::VecDeque;

use tree_sitter::{Node, Parser, Point, Range, Tree};

use crate::grammar::get_language;
use crate::language::Language;
use crate::tokens::CommentToken;
use crate::tree_sitter_extended::{MembershipCheck, RangeFactory, ResolveSymbol};

pub struct Analyzer {
    pub source_code: String,
    pub language: Language,
}

pub trait Traversable<'tree> {
    fn get_indent_comment_pool(&self) -> Vec<String>;
    fn analyze(&self) -> VecDeque<String>;
    fn get_syntax_tree(&self) -> Tree;
    fn get_scannable_nodes(&self, tree: &'tree Tree) -> Vec<(Node<'tree>, (usize, usize, usize))>;
}

impl<'tree> Traversable<'tree> for Analyzer {
    fn get_indent_comment_pool(&self) -> Vec<String> {
        // let comment = match self.language {
        //     Language::Rust => "/// [TODO]",
        //     Language::Python => "# [TODO]",
        //     _ => "//",
        // };

        let comment_token = CommentToken::from_language(&self.language);
        let comment = comment_token.to_str();
        let ident = match self.language {
            Language::Ruby => "  ",
            _ => "    ",
        };
        let max_ident_level = 100;

        (0..max_ident_level)
            .map(|level| {
                let indent = ident.repeat(level);
                format!("{}{}", indent, comment)
            })
            .collect()
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
        let nodes = self.get_scannable_nodes(&tree);

        let ignorable_node_types = self.language.ignorable_node_types();

        let nested_traversable_symbols = self.language.nested_traversable_symbols();

        let mut writer_queue = VecDeque::new();
        let mut pending_queue = VecDeque::new();
        let mut nodes_queue = VecDeque::from(nodes);
        let mut indentation_context: VecDeque<(Node, String)> = VecDeque::new();
        let indent_comment_pool = self.get_indent_comment_pool();
        let mut latest_comment_line = "";
        let mut latest_comment_line_index = -1_isize;

        let mut lines = vec![];
        for line in self.source_code.lines() {
            lines.push(line.to_string());
        }

        for (i, line) in lines.iter().enumerate() {
            let row = i;
            let line_idx = i as isize;
            let column = line.len();

            let cursor_position = Point { row, column };

            if nodes_queue.is_empty() {
                writer_queue.push_back(line.to_owned());
                continue;
            }

            let (current_node, (row, from, to)) = match nodes_queue.front() {
                Some(item) => item,
                None => panic!("Failed to retrieve treesitter node from queue"),
            };

            let mut symbol_name_with_context = String::new();

            let mut pop_node = false;

            match Range::from_node(*current_node) {
                node_range if cursor_position.is_member_of(node_range) => {
                    let node_type = current_node.kind();

                    // rust specific code
                    if node_type == "mod_item" {
                        if node_range.start_point.row == node_range.end_point.row {
                            while !pending_queue.is_empty() {
                                let decorator_line: &str = pending_queue.pop_front().unwrap();
                                writer_queue.push_back(decorator_line.to_owned());
                            }
                            writer_queue.push_back(line.to_owned());
                            nodes_queue.pop_front();
                            continue;
                        }
                    }

                    if ignorable_node_types.contains(&node_type) {
                        while !pending_queue.is_empty() {
                            let decorator_line: &str = pending_queue.pop_front().unwrap();
                            writer_queue.push_back(decorator_line.to_owned());
                        }
                        writer_queue.push_back(line.to_owned());
                        nodes_queue.pop_front();
                        continue;
                    }

                    if node_type == self.language.decorator_node_type() {
                        pending_queue.push_back(line);
                    } else {
                        for (_node, node_symbol) in indentation_context.iter() {
                            symbol_name_with_context
                                .push_str(&format!("{} > ", node_symbol).to_string());
                        }

                        let node_symbol_with_indent = &lines[*row];
                        let node_symbol = &node_symbol_with_indent[from.to_owned()..to.to_owned()];
                        symbol_name_with_context.push_str(node_symbol);

                        let indent_size = indentation_context.len();
                        let comment_line: String = format!(
                            "{} {}",
                            indent_comment_pool[indent_size].clone(),
                            symbol_name_with_context
                        );

                        if latest_comment_line != comment_line {
                            writer_queue.push_back(comment_line);
                        }
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

                    if !indentation_context.is_empty() {
                        if let Some((current_context, _)) = indentation_context.back() {
                            if cursor_position.row >= current_context.end_position().row {
                                indentation_context.pop_back();
                            }
                        }
                    }

                    if nested_traversable_symbols.contains(&node_type) {
                        let (_, from, to) = current_node.identifier_range();
                        indentation_context.push_back((
                            *current_node,
                            line[from.to_owned()..to.to_owned()].to_string(),
                        ));
                        pop_node = true;
                    }

                    if cursor_position == current_node.end_position() {
                        pop_node = true;
                    }

                    if pop_node {
                        nodes_queue.pop_front();
                    }
                }
                _ => {
                    if !indentation_context.is_empty() {
                        if let Some((current_context, _)) = indentation_context.back() {
                            if cursor_position.row >= current_context.end_position().row {
                                indentation_context.pop_back();
                            }
                        }
                    }

                    if line == latest_comment_line && latest_comment_line_index == line_idx - 1 {
                        continue;
                    }

                    let indentation_level = indentation_context.len();
                    if line.starts_with(&indent_comment_pool[indentation_level]) {
                        latest_comment_line = line;
                        latest_comment_line_index = line_idx;
                    }
                    writer_queue.push_back(line.to_owned());
                }
            }
        }

        writer_queue.to_owned()
    }

    /// This methods collects treesitter nodes with BFS
    ///
    /// All of tree sitter nodes are ordered by non decreasing order
    fn get_scannable_nodes(&self, tree: &'tree Tree) -> Vec<(Node<'tree>, (usize, usize, usize))> {
        let mut deq = VecDeque::new();
        let scannable_node_types = self.language.scannable_node_types();
        let commentable_node_types = self.language.commentable_node_types();
        let nested_traversable_symbols = self.language.nested_traversable_symbols();
        let mut result = Vec::new();
        deq.push_back(tree.root_node());

        while !deq.is_empty() {
            if let Some(node) = deq.pop_front() {
                let node_type = node.kind();

                if scannable_node_types.contains(&node_type) {
                    let identifier_range = node.identifier_range();
                    result.push((node, identifier_range));
                }

                if !nested_traversable_symbols.contains(&node_type)
                    && node_type != self.language.top_level_node_type()
                {
                    continue;
                }

                let mut cursor = node.walk();

                if self.language == Language::Ruby {
                    if node_type == self.language.top_level_node_type() {
                        for child_node in node.children(&mut cursor) {
                            if scannable_node_types.contains(&child_node.kind()) {
                                deq.push_back(child_node);
                            }
                        }
                        continue;
                    }
                } else {
                    for child_node in node.children(&mut cursor) {
                        if scannable_node_types.contains(&child_node.kind()) {
                            deq.push_back(child_node);
                        }
                    }
                }

                cursor.reset(node);

                if let Some(body) = node.child_by_field_name("body") {
                    let mut body_cursor = body.walk();
                    for child_node in body.children(&mut body_cursor) {
                        if scannable_node_types.contains(&child_node.kind()) {
                            deq.push_back(child_node);
                        }
                    }
                }
            }
        }

        result.sort_by(|(u, _), (v, _)| u.start_position().row.cmp(&v.start_position().row));

        result.to_owned()
    }
}
