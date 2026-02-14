use std::cell::Cell;

mod visitors;
use visitors::arithmetic_visitor::{count_node, VisitorState};

use arithmetic::*;

#[test]
fn test_visitor_counts_all_nodes() {
    // Expression with 6 operators (+, *, +, -, *, /) and 1 pair of parentheses
    let input = "(1 + 2) * 3 + 4 - 5 * 6 / 7".to_string();
    let mut parser = Parser::new(input).unwrap();
    let root = parser.parse().unwrap();

    let state = VisitorState { count: Cell::new(0) };
    println!("Expression: {}", parser.input());
    parser.arena().visit(root, &mut count_node, Some(&state));

    let total = state.count.get();
    println!("Total nodes visited: {}", total);
    assert!(total > 0, "Should visit at least one node");
    // 1 Expression, 2 AdditiveExpression, 4 MultiplicativeExpression, 9 Primary
    assert_eq!(total, 16, "Node count should match AST structure");
}
