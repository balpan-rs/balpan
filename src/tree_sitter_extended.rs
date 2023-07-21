use tree_sitter::{Point, Range};


pub trait MembershipCheck {
    fn is_member_of(&self, range: Range) -> bool;
}

impl MembershipCheck for Point {
    fn is_member_of(&self, range: Range) -> bool {
        let start_point = range.start_point;
        let end_point = range.end_point;

        if self.row < start_point.row {
            false
        } else if self.row == start_point.row {
            if self.column < start_point.column {
                false
            } else {
                true
            }
        } else {
            if self.row < end_point.row {
                true
            } else if self.row == end_point.row {
                if self.column > end_point.column {
                    false
                } else {
                    true
                }
            } else {
                false
            }
        }
    }
}
