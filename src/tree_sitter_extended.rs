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
        if self.kind() == "attribute_item" {
            return (0, 0, 0);
        }

        let mut node = self.child_by_field_name("name");

        // case of decorated_definition
        if self.kind() == "decorated_definition" {
            let definition_node = self.child_by_field_name("definition").unwrap();
            node = definition_node.child_by_field_name("name");
        }

        // case of impl_item
        if self.kind() == "impl_item" {
            node = self.child_by_field_name("trait");
        }

        let identifier_node = node.unwrap();

        let from = identifier_node.start_position().column;
        let row = identifier_node.end_position().row;
        let to = identifier_node.end_position().column;

        (row, from, to)
    }
}
