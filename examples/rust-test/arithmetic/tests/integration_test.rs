//! Comprehensive integration tests for arithmetic parser
//!
//! Test categories:
//! 1. Positive tests - valid inputs that should parse successfully
//! 2. Negative tests - invalid inputs that should fail with errors
//! 3. AST structure tests - verify the shape of the parsed tree
//! 4. Pretty-print tests - verify pretty-printing of AST

use arithmetic::*;

// ============================================================================
// POSITIVE TESTS - Valid expressions that should parse
// ============================================================================

#[test]
fn test_simple_integer() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "Simple iTODOnteger should parse: {:?}", result.err());
}

#[test]
fn test_single_digit() {
    let mut parser = Parser::new("5".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Single digit should parse");
}

#[test]
fn test_large_integer() {
    let mut parser = Parser::new("123456789".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Large integer should parse");
}

#[test]
fn test_addition() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Addition should parse");
}

#[test]
fn test_subtraction() {
    let mut parser = Parser::new("5 - 3".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Subtraction should parse");
}

#[test]
fn test_multiplication() {
    let mut parser = Parser::new("4 * 6".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Multiplication should parse");
}

#[test]
fn test_division() {
    let mut parser = Parser::new("10 / 2".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Division should parse");
}

#[test]
fn test_precedence_mul_before_add() {
    let mut parser = Parser::new("1 + 2 * 3".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Precedence expression should parse");
}

#[test]
fn test_precedence_div_before_sub() {
    let mut parser = Parser::new("10 - 6 / 2".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Precedence expression should parse");
}

#[test]
fn test_simple_parentheses() {
    let mut parser = Parser::new("(1 + 2)".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Parenthesized expression should parse");
}

#[test]
fn test_parentheses_override_precedence() {
    let mut parser = Parser::new("(1 + 2) * 3".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Parenthesized precedence override should parse");
}

#[test]
fn test_nested_parentheses() {
    let mut parser = Parser::new("((1 + 2))".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Nested parentheses should parse");
}

#[test]
fn test_double_nested_parentheses() {
    let mut parser = Parser::new("(((5)))".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Double nested parentheses should parse");
}

#[test]
fn test_complex_expression() {
    let mut parser = Parser::new("(1 + 2) * 3 - 4 / 5".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Complex expression should parse");
}

#[test]
fn test_multiple_operations() {
    let mut parser = Parser::new("1 + 2 + 3 + 4 + 5".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Multiple additions should parse");
}

#[test]
fn test_mixed_operations() {
    let mut parser = Parser::new("1 + 2 - 3 + 4 - 5".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Mixed add/sub should parse");
}

#[test]
fn test_multiplication_chain() {
    let mut parser = Parser::new("2 * 3 * 4".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Multiplication chain should parse");
}

#[test]
fn test_no_spaces() {
    let mut parser = Parser::new("1+2*3".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Expression without spaces should parse");
}

#[test]
fn test_extra_spaces() {
    let mut parser = Parser::new("  1  +  2  ".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Expression with extra spaces should parse");
}

#[test]
fn test_parentheses_both_sides() {
    let mut parser = Parser::new("((1 + 2) * (3 - 4))".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Parentheses on both sides should parse");
}

// ===========================================d fail");
}

#[test]
fn test_invalid_operator_only() {
    let mut parser = Parser::new("+".to_string(=================================
// NEGATIVE TESTS - Invalid expressions that should fail
// ============================================================================

#[test]
fn test_invalid_missing_operand() {
    let mut parser = Parser::new("1 +".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Missing operand should fail");
}

#[test]
fn test_invalid_missing_operator() {
    let mut parser = Parser::new("1 2".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Missing operator should fail");
}

#[test]
fn test_invalid_unbalanced_open_paren() {
    let mut parser = Parser::new("(1 + 2".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Unbalanced open paren should fail");
}

#[test]
fn test_invalid_unbalanced_close_paren() {
    let mut parser = Parser::new("1 + 2)".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Unbalanced close paren should fail");
}

#[test]
fn test_invalid_empty_parens() {
    let mut parser = Parser::new("()".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Empty parentheses should fail");
}

#[test]
fn test_invalid_operator_start() {
    let mut parser = Parser::new("* 5".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Operator at start should fail");
}

#[test]
fn test_invalid_double_operator() {
    let mut parser = Parser::new("1 ++ 2".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Double operator should fail");
}

#[test]
fn test_invalid_empty_input() {
    let mut parser = Parser::new("".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Empty input should fail");
}

#[test]
fn test_invalid_only_spaces() {
    let mut parser = Parser::new("   ".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Only spaces should fail");
}

#[test]
fn test_invalid_operator_only() {
    let mut parser = Parser::new("+".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Operator only should fail");
}

#[test]
fn test_invalid_trailing_operator() {
    let mut parser = Parser::new("5 *".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Trailing operator should fail");
}

#[test]
fn test_invalid_multiple_close_parens() {
    let mut parser = Parser::new("(1))".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_err(), "Multiple close parens should fail");
}

// ============================================================================
// AST STRUCTURE TESTS - Verify the shape of the parsed tree
// ============================================================================

#[test]
fn test_ast_simple_number() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();

    // Root should be Expression
    match parser.arena().get_node(root) {
        AstNode::Expression(expr) => {
            assert_eq!(expr.children.len(), 1, "Expression should have 1 child");

            // Child should be AdditiveExpression
            let add_id = expr.children[0];
            match parser.arena().get_node(add_id) {
                AstNode::AdditiveExpression(add) => {
                    assert_eq!(add.children.len(), 1, "AdditiveExpression should have 1 child");

                    // Child should be MultiplicativeExpression
                    let mul_id = add.children[0];
                    match parser.arena().get_node(mul_id) {
                        AstNode::MultiplicativeExpression(mul) => {
                            assert_eq!(mul.children.len(), 1, "MultiplicativeExpression should have 1 child");

                            // Child should be Primary with "42"
                            let primary_id = mul.children[0];
                            match parser.arena().get_node(primary_id) {
                                AstNode::Primary(primary) => {
                                    let token = parser.arena().get_token(primary.begin_token);
                                    assert_eq!(token.image, "42", "Primary should contain '42'");
                                }
                                _ => panic!("Expected Primary node"),
                            }
                        }
                        _ => panic!("Expected MultiplicativeExpression node"),
                    }
                }
                _ => panic!("Expected AdditiveExpression node"),
            }
        }
        _ => panic!("Expected Expression node"),
    }
}

#[test]
fn test_ast_addition_has_two_children() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    let root = parser.parse().unwrap();

    match parser.arena().get_node(root) {
        AstNode::Expression(expr) => {
            let add_id = expr.children[0];
            match parser.arena().get_node(add_id) {
                AstNode::AdditiveExpression(add) => {
                    // 1 + 2 should produce 2 children (two MultiplicativeExpressions)
                    assert_eq!(add.children.len(), 2, "1 + 2 should have 2 mult expr children");
                }
                _ => panic!("Expected AdditiveExpression"),
            }
        }
        _ => panic!("Expected Expression"),
    }
}

#[test]
fn test_ast_multiplication_has_two_children() {
    let mut parser = Parser::new("3 * 4".to_string()).unwrap();
    let root = parser.parse().unwrap();

    match parser.arena().get_node(root) {
        AstNode::Expression(expr) => {
            let add_id = expr.children[0];
            match parser.arena().get_node(add_id) {
                AstNode::AdditiveExpression(add) => {
                    let mul_id = add.children[0];
                    match parser.arena().get_node(mul_id) {
                        AstNode::MultiplicativeExpression(mul) => {
                            // 3 * 4 should produce 2 children (two Primaries)
                            assert_eq!(mul.children.len(), 2, "3 * 4 should have 2 primary children");
                        }
                        _ => panic!("Expected MultiplicativeExpression"),
                    }
                }
                _ => panic!("Expected AdditiveExpression"),
            }
        }
        _ => panic!("Expected Expression"),
    }
}

#[test]
fn test_ast_parent_links() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    let root = parser.parse().unwrap();

    match parser.arena().get_node(root) {
        AstNode::Expression(expr) => {
            let add_id = expr.children[0];
            match parser.arena().get_node(add_id) {
                AstNode::AdditiveExpression(add) => {
                    // Verify parent link
                    assert_eq!(add.parent, Some(root), "AdditiveExpression's parent should be root");
                }
                _ => panic!("Expected AdditiveExpression"),
            }
        }
        _ => panic!("Expected Expression"),
    }
}

// ============================================================================
// PRETTY-PRINT TESTS - Verify the pretty-printed AST output
// ============================================================================

#[test]
fn test_pretty_print_simple() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0);

    println!("Pretty print output:\n{}", output);

    assert!(output.contains("Expression"), "Output should contain 'Expression'");
    assert!(output.contains("AdditiveExpression"), "Output should contain 'AdditiveExpression'");
    assert!(output.contains("MultiplicativeExpression"), "Output should contain 'MultiplicativeExpression'");
    assert!(output.contains("Primary"), "Output should contain 'Primary'");
    assert!(output.contains("42"), "Output should contain '42'");
}

#[test]
fn test_pretty_print_addition() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0);

    println!("Pretty print output:\n{}", output);

    // Should have two Primary nodes
    let primary_count = output.matches("Primary").count();
    assert!(primary_count >= 2, "1 + 2 should have at least 2 Primary nodes, found {}", primary_count);
}

#[test]
fn test_pretty_print_complex() {
    let mut parser = Parser::new("(1 + 2) * 3".to_string()).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0);

    println!("Pretty print output:\n{}", output);

    // Should have three Primary nodes for 1, 2, and 3
    let primary_count = output.matches("Primary").count();
    assert!(primary_count >= 3, "(1 + 2) * 3 should have at least 3 Primary nodes, found {}", primary_count);
}

// ============================================================================
// LEXER TESTS - Verify tokenization
// ============================================================================

#[test]
fn test_lexer_tokenization() {
    let mut lexer = Lexer::new("1 + 2".to_string());

    let tok1 = lexer.next_token().unwrap();
    assert_eq!(tok1.token_type, TokenType::INTEGER);
    assert_eq!(tok1.image, "1");

    let tok2 = lexer.next_token().unwrap();
    assert_eq!(tok2.token_type, TokenType::PLUS);
    assert_eq!(tok2.image, "+");

    let tok3 = lexer.next_token().unwrap();
    assert_eq!(tok3.token_type, TokenType::INTEGER);
    assert_eq!(tok3.image, "2");

    let tok4 = lexer.next_token().unwrap();
    assert_eq!(tok4.token_type, TokenType::EOF);
}

#[test]
fn test_lexer_all_operators() {
    let mut lexer = Lexer::new("+ - * / ( )".to_string());

    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::PLUS);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::MINUS);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::STAR);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::SLASH);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::LPAREN);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::RPAREN);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

// ============================================================================
// ARENA TESTS - Verify arena allocation
// ============================================================================

#[test]
fn test_arena_node_allocation() {
    let mut arena = Arena::new();

    let tok = arena.alloc_token(Token::new(
        TokenType::INTEGER,
        "42".to_string(),
        0,
        2
    ));

    let node = arena.alloc_node(AstNode::Primary(PrimaryNode::new(tok, tok)));

    // Verify we can retrieve them
    assert_eq!(arena.get_token(tok).image, "42");
    match arena.get_node(node) {
        AstNode::Primary(primary) => {
            assert_eq!(primary.begin_token, tok);
            assert_eq!(primary.end_token, tok);
        }
        _ => panic!("Expected Primary node"),
    }
}

#[test]
fn test_parser_provides_arena_access() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    let root = parser.parse().unwrap();

    // Verify we can access the arena
    let arena = parser.arena();
    let node = arena.get_node(root);

    match node {
        AstNode::Expression(_) => { /* success */ }
        _ => panic!("Expected Expression at root"),
    }
}

// ============================================================================
// ERROR MESSAGE TESTS - Verify error messages contain useful information
// ============================================================================

#[test]
fn test_error_messages_include_position() {
    let mut parser = Parser::new("* 1".to_string()).unwrap();
    let err = parser.parse().unwrap_err();
    let err_str = format!("{}", err);

    // Error message should include position information
    assert!(err_str.contains("position") || err_str.contains("0"),
            "Error should include position info: {}", err_str);
}

#[test]
fn test_error_messages_include_expected() {
    let mut parser = Parser::new("1 + )".to_string()).unwrap();
    let err = parser.parse().unwrap_err();
    let err_str = format!("{}", err);

    // Error message should indicate what was expected
    assert!(err_str.contains("Expected") || err_str.contains("expected"),
            "Error should indicate expected token: {}", err_str);
}
