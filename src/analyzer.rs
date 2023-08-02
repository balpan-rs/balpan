use std::cell::{RefCell,Ref};
use std::collections::VecDeque;

use tree_sitter::{Tree, Parser, Node, Point, Range};

use crate::grammar::get_language;
use crate::tree_sitter_extended::{MembershipCheck, RangeFactory};

pub struct Analyzer {
    pub source_code: String
}

pub trait Traversable<'a, 'b> {
    fn get_annotation_whitelist(&self) -> Vec<&str>;
    fn analyze(&'b self) -> VecDeque<&'b str>;
    fn get_syntax_tree(&'b self) -> Tree;
    fn get_nested_traversable_symbols(&self) -> Vec<&str>;
    fn get_top_level_nodes(&'a self, tree: &'b Tree) -> Vec<Node<'b>>;
    fn get_whitelist_nodes(&'a self, tree: &'b Tree) -> Vec<Node<'b>>;
}

impl<'a, 'b> Traversable<'a, 'b> for Analyzer {
    fn get_annotation_whitelist(&self) -> Vec<&str> {
        let language = "rust";

        match language {
            "rust" => vec![
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
            _ => vec![],
        }
    }

    fn get_nested_traversable_symbols(&self) -> Vec<&str> {
        let language = "rust";

        match language {
            "rust" => vec![
                "mod_item",
                "impl_item",
            ],
            _ => vec![]
        }
    }

    fn get_syntax_tree(&'b self) -> Tree {
        let parser = RefCell::new(Parser::new());
        let language = get_language("rust").unwrap();

        let mut ts_parser = parser.borrow_mut();
        match ts_parser.set_language(language) {
            Ok(_) => (),
            Err(_) => panic!("treesitter parser for given language does not exists")
        }

        let tree: Option<Tree> = ts_parser.parse(<String as AsRef<str>>::as_ref(&self.source_code.to_string()), None);
        match tree {
            Some(tree) => {
                return tree
            }
            None => panic!("Failed to parsing given source code")
        }
    } 

    fn analyze(&'b self) -> VecDeque<&'b str> {
        let tree = self.get_syntax_tree();
        let nodes = self.get_top_level_nodes(
            &tree
        );

        let whitelist = self.get_annotation_whitelist();

        let writer_queue: &mut VecDeque<&str> = &mut VecDeque::from([]);
        let pending_queue: &mut VecDeque<&str> = &mut VecDeque::from([]);
        let nodes_queue: &mut VecDeque<Node> = &mut VecDeque::from(nodes);
        
        for (i, line) in self.source_code.lines().enumerate() {
            let row    = i;
            let column = line.len();

            let cursor_position = Point {
                row,
                column,
            };

            if nodes_queue.is_empty() {
                writer_queue.push_back(line);
                continue;
            }

            let current_node = match nodes_queue.front() {
                Some(node) => node,
                None => panic!("Failed to retrieve treesitter node from queue")
            };

            let comment_line = "/// [TODO]";
            match Range::from_node(*current_node) {
                node_range if cursor_position.is_member_of(node_range) => {
                    let node_type = &current_node.kind();
                    if whitelist.contains(node_type) {
                        if node_type == &"attribute_item" {
                            pending_queue.push_back(line);
                        } else {
                            if !pending_queue.is_empty() {
                                writer_queue.push_back(comment_line);
                                while !pending_queue.is_empty() {
                                    if let Some(queued_line) = pending_queue.pop_front() {
                                        writer_queue.push_back(queued_line);
                                    }
                                }
                            }
                            writer_queue.push_back(line);
                        }
                    } else {
                        writer_queue.push_back(line);
                    }
                    if cursor_position == current_node.end_position() {
                        nodes_queue.pop_front();
                    }
                },
                _ => {
                    writer_queue.push_back(line);
                }
            }
        }

        writer_queue.to_owned()
    }

    fn get_top_level_nodes(
        &'a self, 
        cloned_tree: &'b Tree
    ) -> Vec<Node<'b>> {
        {
            let node = cloned_tree.root_node();
            let mut cursor = node.walk();

            return node
                .children(&mut cursor)
                .map( |child_node| {
                    child_node
                })
                .collect()

        }
    }

    /// This methods collects treesitter nodes with BFS
    ///
    /// All of tree sitter nodes are ordered by non decreasing order
    fn get_whitelist_nodes(
        &'a self, 
        tree: &'b Tree
    ) -> Vec<Node<'b>> {
        let deq: &mut VecDeque<Node> = &mut VecDeque::from([]);
        let whitelist = self.get_annotation_whitelist();
        let nested_traversable_symbols = self.get_nested_traversable_symbols();
        let result: &mut Vec<Node> = &mut vec![];
        deq.push_back(tree.root_node());

        while !deq.is_empty() {
            if let Some(node) = deq.pop_front() {
                let node_type = &node.kind();

                if whitelist.contains(node_type) {
                    result.push(node);
                }

                if !nested_traversable_symbols.contains(node_type) {
                    if node_type != &"source_file" {
                        continue
                    }
                }

                let mut cursor = node.walk();

                for child_node in node.children(&mut cursor).into_iter() {
                    deq.push_back(child_node);
                }

                cursor.reset(node);

                if let Some(body) = node.child_by_field_name("body") {
                    let mut body_cursor = body.walk();
                    for child_node in body.children(&mut body_cursor).into_iter() {
                        deq.push_back(child_node);
                    }
                }
            }
        }

        result.sort_by(|u, v| {
            u.start_position().row.cmp(
                &v.start_position().row
            )
        });

        for node in result.iter() {
            println!("{}", node.kind());
        }

        return result.to_owned();
    }
}
