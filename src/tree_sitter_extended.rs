use tree_sitter::{Point, Range};


pub trait MembershipCheck {
    fn is_member_of(&self, range: Range) -> bool;
}

impl MembershipCheck for Point {
    fn is_member_of(&self, range: Range) -> bool {
        let start_point = range.start_point;
        let end_point = range.end_point;

        if self.row < start_point.row || self.row > end_point.row {
            return false;
        }

        if self.row == start_point.row && self.column < start_point.column {
            return false;
        }

        if self.row == end_point.row && self.column > end_point.column {
            return false;
        }

        true
    }
}
