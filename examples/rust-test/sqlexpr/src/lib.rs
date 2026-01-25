//! SqlExprParser - SQL Expression Parser
//!
//! This parser handles SQL-like filter expressions including:
//! - Boolean operators: AND, OR, NOT
//! - Comparisons: =, <>, <, >, <=, >=
//! - Arithmetic: +, -, *, /
//! - Special operators: LIKE, IN, BETWEEN, IS NULL, IS NOT NULL
//! - Literals: integers, decimals, strings, true, false, null
//! - Identifiers
//!
//! # Example
//!
//! ```no_run
//! use sqlexpr::*;
//!
//! fn main() -> Result<(), ParseError> {
//!     let input = "price > 100 AND status = 'active'".to_string();
//!     let mut parser = Parser::new(input)?;
//!     let root = parser.parse()?;
//!     println!("{}", parser.arena().pretty_print(root, 0, parser.input()));
//!     Ok(())
//! }
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod error;
mod tokens;
mod arena;
mod lexer;
mod parser;

// Re-export public API
pub use error::{ParseError, ParseResult};
pub use tokens::{Token, TokenType, LexicalState, TokenSource};
pub use arena::{Arena, NodeId, TokenId, AstNode, ComparisonOp};
pub use lexer::Lexer;
pub use parser::Parser;

// Re-export AST node types
pub use arena::{
    SqlExpressionNode, OrExpressionNode, AndExpressionNode, NotExpressionNode,
    ComparisonExpressionNode, ValueListNode, AdditiveExpressionNode,
    MultiplicativeExpressionNode, UnaryExpressionNode, PrimaryExpressionNode,
};

// Re-export operator enums
pub use arena::{AdditiveOp, MultiplicativeOp};
