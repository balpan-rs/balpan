use std::cell::{RefCell,Ref};

use tree_sitter::{Tree, Parser, Node};

use crate::grammar::get_language;

pub struct Analyzer {
    pub source_code: String
}

pub trait Traversable<'a, 'b> {
    fn get_annotation_whitelist(&self) -> Vec<&str>;
    fn analyze(&'b self);
    fn get_syntax_tree(&'b self) -> Tree;
    fn get_top_level_nodes(&'a self, tree: &'b Tree) -> Vec<Node<'b>>;
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

    fn analyze(&'b self) {
        let tree = self.get_syntax_tree();
        let nodes = self.get_top_level_nodes(
            &tree
        );
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
}
