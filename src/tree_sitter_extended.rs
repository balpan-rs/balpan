use tree_sitter::{Point, Range};


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
