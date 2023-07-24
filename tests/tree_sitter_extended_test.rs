#[cfg(test)]
mod tree_sitter_extended_tests {
    use tree_sitter::{Point, Range};
    use balpan::tree_sitter_extended::MembershipCheck;

    #[test]
    fn test_out_of_membership() {
        let cursor = Point {
            row: 2,
            column: 10
        };

        let function_scope = Range {
            start_byte: 0,
            end_byte: 0,
            start_point: Point {
                row: 4,
                column: 2,
            },
            end_point: Point {
                row: 10,
                column: 2
            }
        };

        assert!(cursor.is_before(function_scope));
        assert!(!cursor.is_member_of(function_scope));
    }

    #[test]
    fn test_membership_with_inline_code() {
        let cursor = Point {
            row: 2,
            column: 10
        };

        let inlined_scope = Range {
            start_byte: 0,
            end_byte: 0,
            start_point: Point {
                row: 2,
                column: 5,
            },
            end_point: Point {
                row: 2,
                column: 30
            }
        };

        assert!(cursor.is_member_of(inlined_scope));
    }

    #[test]
    fn test_cursor_is_pointing_the_boundary_of_range() {
        let cursor_with_pointing_start = Point {
            row: 2,
            column: 2
        };
        
        let cursor_with_pointing_end = Point {
            row: 30,
            column: 2,
        };

        let function_scope = Range {
            start_byte: 0,
            end_byte: 0,
            start_point: Point {
                row: 2,
                column: 2,
            },
            end_point: Point {
                row: 30,
                column: 2
            }
        };

        assert!(!cursor_with_pointing_start.is_before(function_scope));
        assert!(cursor_with_pointing_start.is_member_of(function_scope));
        assert!(cursor_with_pointing_end.is_member_of(function_scope));
        assert!(!cursor_with_pointing_end.is_after(function_scope));
    }

    #[test]
    fn test_cursor_is_pointing_outside_of_boundary() {
        let left_of_start_point = Point {
            row: 2,
            column: 1
        };
        
        let right_of_end_point = Point {
            row: 30,
            column: 3,
        };

        let function_scope = Range {
            start_byte: 0,
            end_byte: 0,
            start_point: Point {
                row: 2,
                column: 2,
            },
            end_point: Point {
                row: 30,
                column: 2
            }
        };

        assert!(left_of_start_point.is_before(function_scope));
        assert!(!left_of_start_point.is_member_of(function_scope));
        assert!(!right_of_end_point.is_member_of(function_scope));
        assert!(right_of_end_point.is_after(function_scope));
    }
}
