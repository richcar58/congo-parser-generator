use std::cell::Cell;
use std::any::Any;
use parser::{AstNode, Arena, NodeId, VisitControl};

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
        AstNode::JmsSelector(_) => "JmsSelector",
        AstNode::OrExpression(_) => "OrExpression",
        AstNode::AndExpression(_) => "AndExpression",
        AstNode::EqualityExpression(_) => "EqualityExpression",
        AstNode::ComparisonExpression(_) => "ComparisonExpression",
        AstNode::AddExpression(_) => "AddExpression",
        AstNode::MultExpr(_) => "MultExpr",
        AstNode::UnaryExpr(_) => "UnaryExpr",
        AstNode::PrimaryExpr(_) => "PrimaryExpr",
        AstNode::Literal(_) => "Literal",
        AstNode::StringLitteral(_) => "StringLitteral",
        AstNode::Variable(_) => "Variable",
    };

    let indent = "  ".repeat(depth);
    println!("{}Node {}: depth={}, type={}", indent, n, depth, type_name);

    VisitControl::Continue
}
