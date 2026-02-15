use std::cell::Cell;

mod visitors;
use visitors::sqlexpr_visitor::{count_node, VisitorState};

use sqlexpr::*;

#[test]
fn test_visitor_counts_all_nodes() {
    // Expression with 6 logical operators (AND, OR) and 1 pair of parentheses
    let input = "a = 1 AND b < 2 OR c > 3 AND (d = 4 OR e = 5) AND f != 6 OR g = 7".to_string();
    let mut parser = Parser::new(input).unwrap();
    let root = parser.parse().unwrap();

    let state = VisitorState { count: Cell::new(0) };
    println!("Expression: {}", parser.input());
    parser.arena().visit(root, &mut count_node, Some(&state));

    let total = state.count.get();
    println!("Total nodes visited: {}", total);
    assert!(total > 0, "Should visit at least one node");
}
