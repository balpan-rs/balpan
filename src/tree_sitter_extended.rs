use tree_sitter::{Node, Point, Range};

pub trait MembershipCheck {
    fn is_before(&self, range: Range) -> bool;
    fn is_after(&self, range: Range) -> bool;
    fn is_member_of(&self, range: Range) -> bool;
}

impl MembershipCheck for Point {
    fn is_before(&self, range: Range) -> bool {
        let start_point = range.start_point;

        if self.row < start_point.row {
            return true;
        }

        if self.row > start_point.row {
            return false;
        }

        self.column < start_point.column
    }

    fn is_after(&self, range: Range) -> bool {
        let end_point = range.end_point;

        if self.row < end_point.row {
            return false;
        }

        if self.row > end_point.row {
            return true;
        }

        self.column > end_point.column
    }

    fn is_member_of(&self, range: Range) -> bool {
        if self.is_before(range) {
            return false;
        }

        if self.is_after(range) {
            return false;
        }

        true
    }
}

pub trait RangeFactory {
    fn from_node(node: Node) -> Range;
}

impl RangeFactory for Range {
    #[inline]
    fn from_node(node: Node) -> Range {
        Range {
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            start_point: node.start_position(),
            end_point: node.end_position(),
        }
    }
}

pub trait ResolveSymbol {
    fn identifier_range(&self) -> (usize, usize, usize);
}

impl ResolveSymbol for Node<'_> {
    fn identifier_range(&self) -> (usize, usize, usize) {
        let simple_cases = vec![
            "attribute_item", "use_declaration", "macro_invocation", 
            "expression_statement", "foreign_mod_item"
        ];

        if simple_cases.contains(&self.kind()) {
            return (0, 0, 0)
        }

        let mut node = self.child_by_field_name("name");

        if self.kind() == "namespace_definition" {
            if node == None {
                return (0, 0, 0)
            }
        }

        if self.kind() == "function_definition" {
            match self.child_by_field_name("declarator") {
                Some(child) => {
                    node = child.child_by_field_name("declarator");
                },
                None => {}
            }
        }

        if self.kind() == "method_definition" {
            node = self.child_by_field_name("name");
        }

        // case of decorated_definition
        if self.kind() == "decorated_definition" {
            let definition_node = self.child_by_field_name("definition").unwrap();
            node = definition_node.child_by_field_name("name");
        }

        // case of impl_item
        if self.kind() == "impl_item" {
            node = self.child_by_field_name("trait"); // impl Foo for Bar
            node = match node {
                None => self.child_by_field_name("type"), // impl Foo
                result => result,
            }
        }

        // e.g. `export function foo() {}`
        if self.kind() == "export_statement" {
            // this case handles import statement especially `export * from './compiler_facade_interface';` things.
            // I think this is not a good way to handle this case, but I don't know how to handle this case.
            if self.child_by_field_name("source") != None {
                return (0, 0, 0)
            }

            match self.child_by_field_name("declaration") {
                Some(child) => {
                    node = child.child_by_field_name("name");
                },
                None => {}
            }
        }

        let identifier_node = node.expect(format!("`{}` is an invalid identifier node type", self.kind()).as_str());

        let from = identifier_node.start_position().column;
        let row = identifier_node.end_position().row;
        let to = identifier_node.end_position().column;

        (row, from, to)
    }
}
