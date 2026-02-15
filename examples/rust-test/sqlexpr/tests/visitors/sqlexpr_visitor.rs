use std::cell::Cell;
use std::any::Any;
use sqlexpr::{AstNode, Arena, NodeId, VisitControl};

/// Mutable state passed through the visitor's options parameter.
/// Uses Cell for interior mutability since Arena.visit() passes
/// options as &dyn Any (a shared reference).
pub struct VisitorState {
    pub count: Cell<usize>,
}

/// Visitor callback that counts and prints each AST node.
///
/// For each node, prints one indented line:
///   {indent}Node {n}: depth={d}, type={type}
/// where indent is depth * 2 spaces.
pub fn count_node(
    _id: NodeId,
    node: &AstNode,
    _arena: &Arena,
    depth: usize,
    options: Option<&dyn Any>,
) -> VisitControl {
    let state = options.unwrap().downcast_ref::<VisitorState>().unwrap();
    let n = state.count.get() + 1;
    state.count.set(n);

    let type_name = match node {
        AstNode::SqlExpression(_) => "SqlExpression",
        AstNode::OrExpression(_) => "OrExpression",
        AstNode::AndExpression(_) => "AndExpression",
        AstNode::NotExpression(_) => "NotExpression",
        AstNode::ComparisonExpression(_) => "ComparisonExpression",
        AstNode::ValueList(_) => "ValueList",
        AstNode::AdditiveExpression(_) => "AdditiveExpression",
        AstNode::MultiplicativeExpression(_) => "MultiplicativeExpression",
        AstNode::UnaryExpression(_) => "UnaryExpression",
        AstNode::PrimaryExpression(_) => "PrimaryExpression",
    };

    let indent = "  ".repeat(depth);
    println!("{}Node {}: depth={}, type={}", indent, n, depth, type_name);

    VisitControl::Continue
}
