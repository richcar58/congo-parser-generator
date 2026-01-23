//! Comprehensive integration tests for SQL expression parser
//!
//! Test categories:
//! 1. Positive tests - valid SQL expressions that should parse
//! 2. Negative tests - invalid expressions that should fail
//! 3. AST structure tests - verify the shape of the parsed tree
//! 4. Pretty-print tests - verify pretty-printing of AST

use sqlexpr::*;

// ============================================================================
// POSITIVE TESTS - Valid SQL expressions that should parse
// ============================================================================

// Simple comparisons
#[test]
fn test_simple_equality() {
    let mut parser = Parser::new("x = 1".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Simple equality should parse");
}

#[test]
fn test_not_equal() {
    let mut parser = Parser::new("x <> 1".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "<> comparison should parse");
}

#[test]
fn test_not_equal_bang() {
    let mut parser = Parser::new("x != 1".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "!= comparison should parse");
}

#[test]
fn test_less_than() {
    let mut parser = Parser::new("price < 100".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Less than should parse");
}

#[test]
fn test_greater_than() {
    let mut parser = Parser::new("price > 100".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Greater than should parse");
}

#[test]
fn test_less_or_equal() {
    let mut parser = Parser::new("price <= 100".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Less or equal should parse");
}

#[test]
fn test_greater_or_equal() {
    let mut parser = Parser::new("price >= 100".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Greater or equal should parse");
}

// Boolean operators
#[test]
fn test_and_expression() {
    let mut parser = Parser::new("x = 1 AND y = 2".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "AND expression should parse");
}

#[test]
fn test_or_expression() {
    let mut parser = Parser::new("x = 1 OR y = 2".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "OR expression should parse");
}

#[test]
fn test_not_expression() {
    let mut parser = Parser::new("NOT x = 1".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "NOT expression should parse");
}

#[test]
fn test_complex_boolean() {
    let mut parser = Parser::new("(x = 1 OR y = 2) AND NOT z = 3".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Complex boolean should parse");
}

#[test]
fn test_case_insensitive_keywords() {
    let mut parser = Parser::new("x = 1 and y = 2 or z = 3".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Lowercase keywords should parse");
}

// Special operators
#[test]
fn test_like_expression() {
    let mut parser = Parser::new("name LIKE 'foo%'".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "LIKE expression should parse");
}

#[test]
fn test_in_expression() {
    let mut parser = Parser::new("status IN (1, 2, 3)".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "IN expression should parse");
}

#[test]
fn test_in_strings() {
    let mut parser = Parser::new("status IN ('active', 'pending')".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "IN with strings should parse");
}

#[test]
fn test_between_expression() {
    let mut parser = Parser::new("price BETWEEN 10 AND 100".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "BETWEEN expression should parse");
}

#[test]
fn test_is_null() {
    let mut parser = Parser::new("x IS NULL".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "IS NULL should parse");
}

#[test]
fn test_is_not_null() {
    let mut parser = Parser::new("x IS NOT NULL".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "IS NOT NULL should parse");
}

// Literals
#[test]
fn test_integer_literal() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Integer literal should parse");
}

#[test]
fn test_decimal_literal() {
    let mut parser = Parser::new("3.14".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Decimal literal should parse");
}

#[test]
fn test_string_literal() {
    let mut parser = Parser::new("'hello world'".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "String literal should parse");
}

#[test]
fn test_true_literal() {
    let mut parser = Parser::new("true".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "TRUE literal should parse");
}

#[test]
fn test_false_literal() {
    let mut parser = Parser::new("false".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "FALSE literal should parse");
}

#[test]
fn test_null_literal() {
    let mut parser = Parser::new("null".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "NULL literal should parse");
}

// Arithmetic
#[test]
fn test_addition() {
    let mut parser = Parser::new("x + 1".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Addition should parse");
}

#[test]
fn test_subtraction() {
    let mut parser = Parser::new("x - 1".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Subtraction should parse");
}

#[test]
fn test_multiplication() {
    let mut parser = Parser::new("x * 2".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Multiplication should parse");
}

#[test]
fn test_division() {
    let mut parser = Parser::new("x / 2".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Division should parse");
}

#[test]
fn test_unary_minus() {
    let mut parser = Parser::new("-x".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Unary minus should parse");
}

#[test]
fn test_arithmetic_comparison() {
    let mut parser = Parser::new("x + y * 2 > 100".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Arithmetic in comparison should parse");
}

// Complex expressions
#[test]
fn test_complex_expression() {
    let input = "active = true AND (price < 50 OR quantity > 10)".to_string();
    let mut parser = Parser::new(input).unwrap();
    assert!(parser.parse().is_ok(), "Complex expression should parse");
}

#[test]
fn test_nested_parentheses() {
    let mut parser = Parser::new("((x = 1))".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Nested parentheses should parse");
}

#[test]
fn test_realistic_query() {
    let input = "status = 'active' AND price BETWEEN 10 AND 100 AND name LIKE 'test%'".to_string();
    let mut parser = Parser::new(input).unwrap();
    assert!(parser.parse().is_ok(), "Realistic query should parse");
}

// ============================================================================
// NEGATIVE TESTS - Invalid expressions that should fail
// ============================================================================

#[test]
fn test_invalid_empty() {
    let mut parser = Parser::new("".to_string()).unwrap();
    assert!(parser.parse().is_err(), "Empty input should fail");
}

#[test]
fn test_invalid_just_and() {
    let mut parser = Parser::new("AND".to_string()).unwrap();
    assert!(parser.parse().is_err(), "Just AND should fail");
}

#[test]
fn test_invalid_double_operator() {
    let mut parser = Parser::new("x == 1".to_string()).unwrap();
    assert!(parser.parse().is_err(), "Double equals should fail");
}

#[test]
fn test_invalid_missing_rhs() {
    let mut parser = Parser::new("x =".to_string()).unwrap();
    assert!(parser.parse().is_err(), "Missing right side should fail");
}

#[test]
fn test_invalid_missing_lhs() {
    let mut parser = Parser::new("= 1".to_string()).unwrap();
    assert!(parser.parse().is_err(), "Missing left side should fail");
}

#[test]
fn test_invalid_unbalanced_paren() {
    let mut parser = Parser::new("(x = 1".to_string()).unwrap();
    assert!(parser.parse().is_err(), "Unbalanced paren should fail");
}

#[test]
fn test_invalid_empty_parens() {
    let mut parser = Parser::new("()".to_string()).unwrap();
    assert!(parser.parse().is_err(), "Empty parens should fail");
}

#[test]
fn test_invalid_in_no_list() {
    let mut parser = Parser::new("x IN ()".to_string()).unwrap();
    assert!(parser.parse().is_err(), "IN with empty list should fail");
}

#[test]
fn test_invalid_between_missing_and() {
    let mut parser = Parser::new("x BETWEEN 1 2".to_string()).unwrap();
    assert!(parser.parse().is_err(), "BETWEEN without AND should fail");
}

#[test]
fn test_invalid_like_no_string() {
    let mut parser = Parser::new("x LIKE 123".to_string()).unwrap();
    assert!(parser.parse().is_err(), "LIKE with non-string should fail");
}

#[test]
fn test_invalid_trailing_and() {
    let mut parser = Parser::new("x = 1 AND".to_string()).unwrap();
    assert!(parser.parse().is_err(), "Trailing AND should fail");
}

#[test]
fn test_invalid_unterminated_string() {
    // Unterminated string is detected during lexer initialization
    let result = Parser::new("'unterminated".to_string());
    assert!(result.is_err(), "Unterminated string should fail during lexer init");
}

// ============================================================================
// AST STRUCTURE TESTS - Verify the shape of the parsed tree
// ============================================================================

#[test]
fn test_ast_simple_comparison() {
    let mut parser = Parser::new("x = 1".to_string()).unwrap();
    let root = parser.parse().unwrap();

    match parser.arena().get_node(root) {
        AstNode::SqlExpression(expr) => {
            assert_eq!(expr.children.len(), 1, "SqlExpression should have 1 child");
        }
        _ => panic!("Expected SqlExpression"),
    }
}

#[test]
fn test_ast_and_has_two_children() {
    let mut parser = Parser::new("x = 1 AND y = 2".to_string()).unwrap();
    let root = parser.parse().unwrap();

    // Walk down to the AndExpression
    match parser.arena().get_node(root) {
        AstNode::SqlExpression(sql) => {
            let or_id = sql.children[0];
            match parser.arena().get_node(or_id) {
                AstNode::OrExpression(or) => {
                    let and_id = or.children[0];
                    match parser.arena().get_node(and_id) {
                        AstNode::AndExpression(and) => {
                            assert_eq!(and.children.len(), 2, "AND should have 2 children");
                        }
                        _ => panic!("Expected AndExpression"),
                    }
                }
                _ => panic!("Expected OrExpression"),
            }
        }
        _ => panic!("Expected SqlExpression"),
    }
}

#[test]
fn test_ast_comparison_op() {
    let mut parser = Parser::new("x > 5".to_string()).unwrap();
    let root = parser.parse().unwrap();

    // Navigate to ComparisonExpression - helper to search children
    fn search_children(arena: &Arena, children: &[NodeId]) -> Option<ComparisonOp> {
        for child in children {
            if let Some(op) = find_comparison(arena, *child) {
                return Some(op);
            }
        }
        None
    }

    fn find_comparison(arena: &Arena, id: NodeId) -> Option<ComparisonOp> {
        match arena.get_node(id) {
            AstNode::ComparisonExpression(node) => node.comparison_op,
            AstNode::SqlExpression(node) => search_children(arena, &node.children),
            AstNode::OrExpression(node) => search_children(arena, &node.children),
            AstNode::AndExpression(node) => search_children(arena, &node.children),
            AstNode::NotExpression(node) => search_children(arena, &node.children),
            AstNode::ValueList(node) => search_children(arena, &node.children),
            AstNode::AdditiveExpression(node) => search_children(arena, &node.children),
            AstNode::MultiplicativeExpression(node) => search_children(arena, &node.children),
            AstNode::UnaryExpression(node) => search_children(arena, &node.children),
            AstNode::PrimaryExpression(node) => search_children(arena, &node.children),
        }
    }

    let op = find_comparison(parser.arena(), root);
    assert_eq!(op, Some(ComparisonOp::Gt), "Should have GT comparison");
}

// ============================================================================
// PRETTY-PRINT TESTS - Verify the pretty-printed AST output
// ============================================================================

#[test]
fn test_pretty_print_simple() {
    let mut parser = Parser::new("x = 1".to_string()).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0);

    println!("Pretty print output:\n{}", output);

    assert!(output.contains("SqlExpression"), "Should contain SqlExpression");
    assert!(output.contains("ComparisonExpression"), "Should contain ComparisonExpression");
    assert!(output.contains("PrimaryExpression"), "Should contain PrimaryExpression");
}

#[test]
fn test_pretty_print_and() {
    let mut parser = Parser::new("x = 1 AND y = 2".to_string()).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0);

    println!("Pretty print output:\n{}", output);

    assert!(output.contains("AndExpression"), "Should contain AndExpression");
    // Should have at least 2 PrimaryExpression nodes (x and y, or more)
    let primary_count = output.matches("PrimaryExpression").count();
    assert!(primary_count >= 2, "Should have at least 2 PrimaryExpressions, found {}", primary_count);
}

#[test]
fn test_pretty_print_complex() {
    let input = "active = true AND price > 100".to_string();
    let mut parser = Parser::new(input).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0);

    println!("Pretty print output:\n{}", output);

    assert!(output.contains("SqlExpression"), "Should contain SqlExpression");
    assert!(output.contains("AndExpression"), "Should contain AndExpression");
}

// ============================================================================
// LEXER TESTS - Verify tokenization
// ============================================================================

#[test]
fn test_lexer_keywords() {
    let mut lexer = Lexer::new("AND OR NOT".to_string());

    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::AND);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::OR);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::NOT);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_operators() {
    let mut lexer = Lexer::new("= <> < > <= >= + - * /".to_string());

    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EQ);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::NE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::LT);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::GT);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::LE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::GE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::PLUS);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::MINUS);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::STAR);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::SLASH);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_literals() {
    let mut lexer = Lexer::new("42 3.14 'hello' true false null".to_string());

    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::INTEGER_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::STRING_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::TRUE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::FALSE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::NULL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_identifiers() {
    let mut lexer = Lexer::new("foo bar_baz _private".to_string());

    let t1 = lexer.next_token().unwrap();
    assert_eq!(t1.token_type, TokenType::IDENTIFIER);
    assert_eq!(t1.image, "foo");

    let t2 = lexer.next_token().unwrap();
    assert_eq!(t2.token_type, TokenType::IDENTIFIER);
    assert_eq!(t2.image, "bar_baz");

    let t3 = lexer.next_token().unwrap();
    assert_eq!(t3.token_type, TokenType::IDENTIFIER);
    assert_eq!(t3.image, "_private");
}

// ============================================================================
// ARENA TESTS - Verify arena allocation
// ============================================================================

#[test]
fn test_arena_node_allocation() {
    let mut arena = Arena::new();

    let tok = arena.alloc_token(Token::new(
        TokenType::IDENTIFIER,
        "test".to_string(),
        0,
        4,
    ));

    let node = arena.alloc_node(AstNode::PrimaryExpression(PrimaryExpressionNode::new(tok, tok)));

    assert_eq!(arena.get_token(tok).image, "test");
    match arena.get_node(node) {
        AstNode::PrimaryExpression(primary) => {
            assert_eq!(primary.begin_token, tok);
        }
        _ => panic!("Expected PrimaryExpression"),
    }
}

#[test]
fn test_parser_provides_arena_access() {
    let mut parser = Parser::new("x = 1".to_string()).unwrap();
    let root = parser.parse().unwrap();

    let arena = parser.arena();
    match arena.get_node(root) {
        AstNode::SqlExpression(_) => { /* success */ }
        _ => panic!("Expected SqlExpression at root"),
    }
}

// ============================================================================
// ERROR MESSAGE TESTS - Verify error messages contain useful information
// ============================================================================

#[test]
fn test_error_messages_include_position() {
    let mut parser = Parser::new("= 1".to_string()).unwrap();
    let err = parser.parse().unwrap_err();
    let err_str = format!("{}", err);

    assert!(
        err_str.contains("position") || err_str.contains("0") || err_str.contains("Expected"),
        "Error should include position info: {}",
        err_str
    );
}

#[test]
fn test_error_messages_include_expected() {
    let mut parser = Parser::new("x =".to_string()).unwrap();
    let err = parser.parse().unwrap_err();
    let err_str = format!("{}", err);

    assert!(
        err_str.contains("Expected") || err_str.contains("expected"),
        "Error should indicate expected token: {}",
        err_str
    );
}
