//! Comprehensive integration tests for the sqlexpr2 parser
//!
//! This grammar is a JMS Selector expression parser (from Apache ActiveMQ).
//! It supports: boolean logic (AND, OR, NOT), comparisons (=, <>, >, >=, <, <=),
//! IS NULL / IS NOT NULL, LIKE, BETWEEN, IN, arithmetic (+, -, *, /, %),
//! unary operators (+, -, NOT), literals (string, decimal, hex, octal, float,
//! TRUE, FALSE, NULL), identifiers, and parenthesized subexpressions.
//!
//! Test categories:
//! 1. Lexer - tokenization of operators, keywords, literals, whitespace
//! 2. Parser positive - valid expressions that should parse with correct AST
//! 3. Parser negative - invalid expressions that should fail
//! 4. Edge cases - boundary conditions and unusual inputs
//! 5. Arena - allocation, mutation, node types
//! 6. Visitor - depth-first traversal, controls, options
//! 7. Pretty-print - output format
//! 8. Error types - ParseError API

use parser::*;
use std::any::Any;

// ============================================================================
// LEXER TESTS - Keywords
// ============================================================================

#[test]
fn test_lexer_all_keywords_uppercase() {
    let mut lexer = Lexer::new("NOT AND OR BETWEEN LIKE ESCAPE IN IS TRUE FALSE NULL".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::NOT);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::AND);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::OR);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::BETWEEN);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::LIKE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::ESCAPE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::IN);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::IS);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::TRUE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::FALSE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::NULL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_all_keywords_lowercase() {
    let mut lexer = Lexer::new("not and or between like escape in is true false null".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::NOT);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::AND);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::OR);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::BETWEEN);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::LIKE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::ESCAPE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::IN);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::IS);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::TRUE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::FALSE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::NULL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_keywords_mixed_case() {
    let mut lexer = Lexer::new("And Or Not Between".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::AND);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::OR);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::NOT);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::BETWEEN);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_keyword_preserves_original_image() {
    let mut lexer = Lexer::new("aNd".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::AND);
    assert_eq!(tok.image, "aNd", "Token image should preserve original case");
}

// ============================================================================
// LEXER TESTS - Single-character operators
// ============================================================================

#[test]
fn test_lexer_single_char_operators() {
    let mut lexer = Lexer::new("= > < ( ) , + - * / %".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token17); // =
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token19); // >
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token21); // <
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token23); // (
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token25); // )
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token24); // ,
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token26); // +
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token27); // -
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token28); // *
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token29); // /
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token30); // %
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_operator_images() {
    let mut lexer = Lexer::new("= > < ( ) , + - * / %".to_string());
    assert_eq!(lexer.next_token().unwrap().image, "=");
    assert_eq!(lexer.next_token().unwrap().image, ">");
    assert_eq!(lexer.next_token().unwrap().image, "<");
    assert_eq!(lexer.next_token().unwrap().image, "(");
    assert_eq!(lexer.next_token().unwrap().image, ")");
    assert_eq!(lexer.next_token().unwrap().image, ",");
    assert_eq!(lexer.next_token().unwrap().image, "+");
    assert_eq!(lexer.next_token().unwrap().image, "-");
    assert_eq!(lexer.next_token().unwrap().image, "*");
    assert_eq!(lexer.next_token().unwrap().image, "/");
    assert_eq!(lexer.next_token().unwrap().image, "%");
}

// ============================================================================
// LEXER TESTS - Multi-character operators
// ============================================================================

#[test]
fn test_lexer_multi_char_operators() {
    let mut lexer = Lexer::new("<> >= <=".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token18); // <>
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token20); // >=
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token22); // <=
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_multi_char_operator_images() {
    let mut lexer = Lexer::new("<> >= <=".to_string());
    assert_eq!(lexer.next_token().unwrap().image, "<>");
    assert_eq!(lexer.next_token().unwrap().image, ">=");
    assert_eq!(lexer.next_token().unwrap().image, "<=");
}

#[test]
fn test_lexer_multi_char_operator_positions() {
    let mut lexer = Lexer::new("<>".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.begin_offset, 0);
    assert_eq!(tok.end_offset, 2);
    assert_eq!(tok.image, "<>");
}

#[test]
fn test_lexer_longest_match_le() {
    // "<=" should be one token, not "<" + "="
    let mut lexer = Lexer::new("<=".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::Token22);
    assert_eq!(tok.image, "<=");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_longest_match_ge() {
    let mut lexer = Lexer::new(">=".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::Token20);
    assert_eq!(tok.image, ">=");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_longest_match_ne() {
    let mut lexer = Lexer::new("<>".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::Token18);
    assert_eq!(tok.image, "<>");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_lt_space_eq_are_two_tokens() {
    // "< =" with a space should be two separate tokens
    let mut lexer = Lexer::new("< =".to_string());
    let t1 = lexer.next_token().unwrap();
    assert_eq!(t1.token_type, TokenType::Token21); // <
    let t2 = lexer.next_token().unwrap();
    assert_eq!(t2.token_type, TokenType::Token17); // =
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

// ============================================================================
// LEXER TESTS - String literals
// ============================================================================

#[test]
fn test_lexer_string_literal() {
    let mut lexer = Lexer::new("'hello world'".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::STRING_LITERAL);
    assert_eq!(tok.image, "'hello world'");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_empty_string_literal() {
    let mut lexer = Lexer::new("''".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::STRING_LITERAL);
    assert_eq!(tok.image, "''");
}

#[test]
fn test_lexer_string_with_special_chars() {
    let mut lexer = Lexer::new("'hello%world_test'".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::STRING_LITERAL);
    assert_eq!(tok.image, "'hello%world_test'");
}

#[test]
fn test_lexer_string_with_spaces() {
    let mut lexer = Lexer::new("'hello   world   foo'".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::STRING_LITERAL);
    assert_eq!(tok.image, "'hello   world   foo'");
}

#[test]
fn test_lexer_string_with_digits() {
    let mut lexer = Lexer::new("'abc123'".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::STRING_LITERAL);
    assert_eq!(tok.image, "'abc123'");
}

#[test]
fn test_lexer_multiple_strings() {
    let mut lexer = Lexer::new("'a' 'b' 'c'".to_string());
    assert_eq!(lexer.next_token().unwrap().image, "'a'");
    assert_eq!(lexer.next_token().unwrap().image, "'b'");
    assert_eq!(lexer.next_token().unwrap().image, "'c'");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_unterminated_string() {
    let mut lexer = Lexer::new("'unterminated".to_string());
    let result = lexer.next_token();
    assert!(result.is_err(), "Unterminated string should produce an error");
}

// ============================================================================
// LEXER TESTS - Numeric literals
// ============================================================================

#[test]
fn test_lexer_integer() {
    let mut lexer = Lexer::new("42".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "42");
}

#[test]
fn test_lexer_zero() {
    let mut lexer = Lexer::new("0".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "0");
}

#[test]
fn test_lexer_large_number() {
    let mut lexer = Lexer::new("999999999".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "999999999");
}

#[test]
fn test_lexer_decimal() {
    let mut lexer = Lexer::new("3.14".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "3.14");
}

#[test]
fn test_lexer_decimal_no_fraction() {
    // "5." — integer followed by dot, but no digit after dot → just integer "5"
    let mut lexer = Lexer::new("5.".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "5");
}

#[test]
fn test_lexer_multiple_numbers() {
    let mut lexer = Lexer::new("1 22 333".to_string());
    assert_eq!(lexer.next_token().unwrap().image, "1");
    assert_eq!(lexer.next_token().unwrap().image, "22");
    assert_eq!(lexer.next_token().unwrap().image, "333");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

// ============================================================================
// LEXER TESTS - Whitespace handling
// ============================================================================

#[test]
fn test_lexer_skips_spaces() {
    let mut lexer = Lexer::new("   42   ".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "42");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_skips_tabs() {
    let mut lexer = Lexer::new("\t42\t".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "42");
}

#[test]
fn test_lexer_skips_newlines() {
    let mut lexer = Lexer::new("\n42\n".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "42");
}

#[test]
fn test_lexer_skips_carriage_return() {
    let mut lexer = Lexer::new("\r42\r".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "42");
}

#[test]
fn test_lexer_skips_mixed_whitespace() {
    let mut lexer = Lexer::new(" \t\n\r 42 \t\n\r ".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(tok.image, "42");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

// ============================================================================
// LEXER TESTS - EOF handling
// ============================================================================

#[test]
fn test_lexer_eof_on_empty_input() {
    let mut lexer = Lexer::new("".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_eof_on_whitespace_only() {
    let mut lexer = Lexer::new("   \t\n   ".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_repeated_eof() {
    let mut lexer = Lexer::new("42".to_string());
    let _ = lexer.next_token().unwrap(); // consume 42
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

// ============================================================================
// LEXER TESTS - Token positions
// ============================================================================

#[test]
fn test_lexer_token_positions() {
    let mut lexer = Lexer::new("42 = 99".to_string());
    let t1 = lexer.next_token().unwrap();
    assert_eq!(t1.begin_offset, 0);
    assert_eq!(t1.end_offset, 2);

    let t2 = lexer.next_token().unwrap();
    assert_eq!(t2.begin_offset, 3);
    assert_eq!(t2.end_offset, 4);

    let t3 = lexer.next_token().unwrap();
    assert_eq!(t3.begin_offset, 5);
    assert_eq!(t3.end_offset, 7);
}

#[test]
fn test_lexer_string_literal_positions() {
    let mut lexer = Lexer::new("'hello'".to_string());
    let tok = lexer.next_token().unwrap();
    assert_eq!(tok.begin_offset, 0);
    assert_eq!(tok.end_offset, 7);
}

// ============================================================================
// LEXER TESTS - Error cases
// ============================================================================

#[test]
fn test_lexer_unknown_char_at() {
    let mut lexer = Lexer::new("@".to_string());
    assert!(lexer.next_token().is_err(), "@ should produce a lexer error");
}

#[test]
fn test_lexer_unknown_char_hash() {
    let mut lexer = Lexer::new("#".to_string());
    assert!(lexer.next_token().is_err(), "# should produce a lexer error");
}

#[test]
fn test_lexer_unknown_char_caret() {
    let mut lexer = Lexer::new("^".to_string());
    assert!(lexer.next_token().is_err(), "^ should produce a lexer error");
}

#[test]
fn test_lexer_unknown_char_bang() {
    // Grammar uses <> for not-equal, not !=, so ! is not a valid token
    let mut lexer = Lexer::new("!".to_string());
    assert!(lexer.next_token().is_err(), "! should produce a lexer error");
}

// ============================================================================
// LEXER TESTS - Identifier handling (known limitation)
// The grammar defines <ID> but the generated lexer only has match_keyword,
// not match_identifier_or_keyword. Non-keyword identifiers are consumed by
// match_keyword but it returns Ok(None), causing a confusing error.
// ============================================================================

#[test]
fn test_lexer_non_keyword_identifier_fails() {
    let mut lexer = Lexer::new("foo".to_string());
    let result = lexer.next_token();
    // "foo" is consumed by match_keyword (alphabetic chars), but doesn't match
    // any keyword. The function returns Ok(None), which causes next_token to
    // report "Unexpected character" (at the wrong position since chars were consumed).
    assert!(
        result.is_err(),
        "Non-keyword identifier should error (ID token not supported in generated lexer)"
    );
}

#[test]
fn test_lexer_dollar_identifier_fails() {
    // Grammar defines ID as starting with ["a"-"z", "_", "$"] but $ is not
    // recognized by the generated keyword matcher
    let mut lexer = Lexer::new("$foo".to_string());
    assert!(
        lexer.next_token().is_err(),
        "$ prefix identifier should error (ID token not supported)"
    );
}

// ============================================================================
// LEXER TESTS - Combined token sequences
// ============================================================================

#[test]
fn test_lexer_comparison_expression_tokens() {
    let mut lexer = Lexer::new("42 >= 10".to_string());
    let t1 = lexer.next_token().unwrap();
    assert_eq!(t1.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(t1.image, "42");

    let t2 = lexer.next_token().unwrap();
    assert_eq!(t2.token_type, TokenType::Token20); // >=
    assert_eq!(t2.image, ">=");

    let t3 = lexer.next_token().unwrap();
    assert_eq!(t3.token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(t3.image, "10");

    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_string_comma_sequence() {
    let mut lexer = Lexer::new("'a','b','c'".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::STRING_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token24); // ,
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::STRING_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token24); // ,
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::STRING_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_operators_no_spaces() {
    let mut lexer = Lexer::new("42=99".to_string());
    assert_eq!(lexer.next_token().unwrap().image, "42");
    assert_eq!(lexer.next_token().unwrap().image, "=");
    assert_eq!(lexer.next_token().unwrap().image, "99");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_keyword_between_operators() {
    let mut lexer = Lexer::new("TRUE AND FALSE".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::TRUE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::AND);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::FALSE);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_parenthesized_expression() {
    let mut lexer = Lexer::new("(42)".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token23); // (
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token25); // )
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_lexer_in_list_expression() {
    let mut lexer = Lexer::new("IN ('a', 'b')".to_string());
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::IN);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token23); // (
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::STRING_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token24); // ,
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::STRING_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token25); // )
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

// ============================================================================
// PARSER TESTS - Construction
// ============================================================================

#[test]
fn test_parser_new_succeeds_on_valid_input() {
    let result = Parser::new("42".to_string());
    assert!(result.is_ok(), "Parser::new should succeed on valid input");
}

#[test]
fn test_parser_new_succeeds_on_keyword() {
    let result = Parser::new("TRUE".to_string());
    assert!(result.is_ok(), "Parser::new should succeed with keyword input");
}

#[test]
fn test_parser_new_succeeds_on_string_literal() {
    let result = Parser::new("'hello'".to_string());
    assert!(result.is_ok(), "Parser::new should succeed with string literal");
}

#[test]
fn test_parser_new_fails_on_unterminated_string() {
    let result = Parser::new("'unterminated".to_string());
    assert!(
        result.is_err(),
        "Parser::new should fail when first token is an unterminated string"
    );
}

#[test]
fn test_parser_new_succeeds_on_empty() {
    // Empty input produces an EOF token, which is valid for Parser::new
    let result = Parser::new("".to_string());
    assert!(result.is_ok(), "Parser::new should succeed on empty input (EOF token)");
}

#[test]
fn test_parser_input_accessor() {
    let parser = Parser::new("42".to_string()).unwrap();
    assert_eq!(parser.input(), "42");
}

#[test]
fn test_parser_arena_accessor() {
    let parser = Parser::new("42".to_string()).unwrap();
    let _arena = parser.arena();
    // Just verify it doesn't panic
}

// ============================================================================
// PARSER TESTS - Positive (parse returns a root node)
// ============================================================================

#[test]
fn test_parse_returns_root_node() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let result = parser.parse();
    assert!(result.is_ok(), "parse() should return Ok with a root node");
}

#[test]
fn test_parse_root_is_jms_selector() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(_) => { /* correct */ }
        other => panic!("Expected JmsSelector at root, got {:?}", other),
    }
}

#[test]
fn test_parse_true_literal() {
    let mut parser = Parser::new("TRUE".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "TRUE should parse successfully");
}

#[test]
fn test_parse_false_literal() {
    let mut parser = Parser::new("FALSE".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "FALSE should parse successfully");
}

#[test]
fn test_parse_null_literal() {
    let mut parser = Parser::new("NULL".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "NULL should parse successfully");
}

#[test]
fn test_parse_string_literal() {
    let mut parser = Parser::new("'hello'".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "String literal should parse");
}

#[test]
fn test_parse_integer_literal() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Integer literal should parse");
}

#[test]
fn test_parse_decimal_literal() {
    let mut parser = Parser::new("3.14".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Decimal literal should parse");
}

// ============================================================================
// PARSER TESTS - AST structure (requires actual recursive descent parsing)
// These test that the parser produces meaningful AST trees, not just a
// single stub node.
// ============================================================================

#[test]
fn test_parse_equality_has_children() {
    let mut parser = Parser::new("42 = 99".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "JmsSelector for '42 = 99' should have child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_and_expression_has_children() {
    let mut parser = Parser::new("TRUE AND FALSE".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "AND expression should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_or_expression_has_children() {
    let mut parser = Parser::new("TRUE OR FALSE".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "OR expression should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_not_expression_has_children() {
    let mut parser = Parser::new("NOT TRUE".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "NOT expression should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_addition_has_children() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "Addition should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_multiplication_has_children() {
    let mut parser = Parser::new("2 * 3".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "Multiplication should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_comparison_gt_has_children() {
    let mut parser = Parser::new("42 > 10".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "Comparison should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_parenthesized_has_children() {
    let mut parser = Parser::new("(42)".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "Parenthesized expression should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_complex_arithmetic_has_children() {
    let mut parser = Parser::new("1 + 2 * 3 - 4 / 2".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "Complex arithmetic should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_unary_minus_has_children() {
    let mut parser = Parser::new("-42".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "Unary minus should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_modulo_has_children() {
    let mut parser = Parser::new("10 % 3".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "Modulo should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_between_has_children() {
    let mut parser = Parser::new("42 BETWEEN 1 AND 100".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "BETWEEN should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

#[test]
fn test_parse_complex_boolean_has_children() {
    let mut parser = Parser::new("(TRUE OR FALSE) AND NOT TRUE".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(node) => {
            assert!(
                !node.children.is_empty(),
                "Complex boolean should produce child nodes (actual: empty — parser is a stub)"
            );
        }
        _ => panic!("Expected JmsSelector at root"),
    }
}

// ============================================================================
// PARSER NEGATIVE TESTS - Invalid inputs that should fail
// ============================================================================

#[test]
fn test_parse_empty_should_fail() {
    let mut parser = Parser::new("".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "Empty input should fail to parse (should require at least one expression)"
    );
}

#[test]
fn test_parse_just_operator_should_fail() {
    let mut parser = Parser::new("=".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "Bare '=' operator should fail to parse"
    );
}

#[test]
fn test_parse_just_and_keyword_should_fail() {
    let mut parser = Parser::new("AND".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "Bare AND keyword is not a valid expression"
    );
}

#[test]
fn test_parse_just_or_keyword_should_fail() {
    let mut parser = Parser::new("OR".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "Bare OR keyword is not a valid expression"
    );
}

#[test]
fn test_parse_trailing_operator_should_fail() {
    let mut parser = Parser::new("42 +".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "Trailing + operator should fail (missing right operand)"
    );
}

#[test]
fn test_parse_unbalanced_open_paren_should_fail() {
    let mut parser = Parser::new("(42".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "Unbalanced open paren should fail"
    );
}

#[test]
fn test_parse_unbalanced_close_paren_should_fail() {
    let mut parser = Parser::new("42)".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "Unexpected close paren should fail"
    );
}

#[test]
fn test_parse_empty_parens_should_fail() {
    let mut parser = Parser::new("()".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "Empty parentheses should fail"
    );
}

#[test]
fn test_parse_double_operator_is_unary_plus() {
    // "42 ++ 1" is valid: it parses as "42 + (+1)" since + is both binary and unary
    let mut parser = Parser::new("42 ++ 1".to_string()).unwrap();
    assert!(
        parser.parse().is_ok(),
        "42 ++ 1 should succeed (binary + followed by unary +)"
    );
}

#[test]
fn test_parse_consecutive_literals_should_fail() {
    let mut parser = Parser::new("42 99".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "Two consecutive literals without operator should fail"
    );
}

#[test]
fn test_parse_between_missing_and_should_fail() {
    let mut parser = Parser::new("42 BETWEEN 1 100".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "BETWEEN without AND should fail"
    );
}

#[test]
fn test_parse_in_empty_list_should_fail() {
    let mut parser = Parser::new("42 IN ()".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "IN with empty list should fail"
    );
}

#[test]
fn test_parse_like_without_pattern_should_fail() {
    let mut parser = Parser::new("42 LIKE".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "LIKE without pattern should fail"
    );
}

#[test]
fn test_parse_is_without_null_should_fail() {
    let mut parser = Parser::new("42 IS 99".to_string()).unwrap();
    assert!(
        parser.parse().is_err(),
        "IS without NULL should fail"
    );
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_lots_of_whitespace() {
    let mut parser = Parser::new("   42   ".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Input with lots of whitespace should parse");
}

#[test]
fn test_tabs_and_newlines() {
    let mut parser = Parser::new("\t\n42\t\n".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Input with tabs/newlines should parse");
}

#[test]
fn test_single_char_number() {
    let mut parser = Parser::new("0".to_string()).unwrap();
    assert!(parser.parse().is_ok(), "Single digit should parse");
}

#[test]
fn test_deeply_nested_parentheses() {
    // Even though parser is a stub, this tests lexer tokenization
    let input = "((((42))))".to_string();
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token23); // (
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token23); // (
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token23); // (
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token23); // (
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::DECIMAL_LITERAL);
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token25); // )
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token25); // )
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token25); // )
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::Token25); // )
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

#[test]
fn test_many_operators_in_sequence() {
    let input = "1 + 2 - 3 * 4 / 5 % 6".to_string();
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next_token().unwrap().image, "1");
    assert_eq!(lexer.next_token().unwrap().image, "+");
    assert_eq!(lexer.next_token().unwrap().image, "2");
    assert_eq!(lexer.next_token().unwrap().image, "-");
    assert_eq!(lexer.next_token().unwrap().image, "3");
    assert_eq!(lexer.next_token().unwrap().image, "*");
    assert_eq!(lexer.next_token().unwrap().image, "4");
    assert_eq!(lexer.next_token().unwrap().image, "/");
    assert_eq!(lexer.next_token().unwrap().image, "5");
    assert_eq!(lexer.next_token().unwrap().image, "%");
    assert_eq!(lexer.next_token().unwrap().image, "6");
    assert_eq!(lexer.next_token().unwrap().token_type, TokenType::EOF);
}

// ============================================================================
// TOKEN API TESTS
// ============================================================================

#[test]
fn test_token_len() {
    let tok = Token::new(TokenType::DECIMAL_LITERAL, "42".to_string(), 0, 2);
    assert_eq!(tok.len(), 2);
}

#[test]
fn test_token_is_empty() {
    let tok = Token::new(TokenType::EOF, "".to_string(), 5, 5);
    assert!(tok.is_empty());
}

#[test]
fn test_token_is_not_empty() {
    let tok = Token::new(TokenType::DECIMAL_LITERAL, "42".to_string(), 0, 2);
    assert!(!tok.is_empty());
}

#[test]
fn test_token_is_type() {
    let tok = Token::new(TokenType::AND, "AND".to_string(), 0, 3);
    assert!(tok.is_type(TokenType::AND));
    assert!(!tok.is_type(TokenType::OR));
}

#[test]
fn test_token_is_one_of() {
    let tok = Token::new(TokenType::AND, "AND".to_string(), 0, 3);
    assert!(tok.is_one_of(&[TokenType::AND, TokenType::OR]));
    assert!(!tok.is_one_of(&[TokenType::NOT, TokenType::OR]));
}

#[test]
fn test_token_display() {
    let tok = Token::new(TokenType::AND, "AND".to_string(), 0, 3);
    let display = format!("{}", tok);
    assert!(display.contains("AND"), "Token display should contain type name");
    assert!(display.contains("AND"), "Token display should contain image");
}

// ============================================================================
// TOKEN TYPE DISPLAY TESTS
// ============================================================================

#[test]
fn test_token_type_display() {
    assert_eq!(format!("{}", TokenType::EOF), "EOF");
    assert_eq!(format!("{}", TokenType::AND), "AND");
    assert_eq!(format!("{}", TokenType::OR), "OR");
    assert_eq!(format!("{}", TokenType::NOT), "NOT");
    assert_eq!(format!("{}", TokenType::BETWEEN), "BETWEEN");
    assert_eq!(format!("{}", TokenType::LIKE), "LIKE");
    assert_eq!(format!("{}", TokenType::ESCAPE), "ESCAPE");
    assert_eq!(format!("{}", TokenType::IN), "IN");
    assert_eq!(format!("{}", TokenType::IS), "IS");
    assert_eq!(format!("{}", TokenType::TRUE), "TRUE");
    assert_eq!(format!("{}", TokenType::FALSE), "FALSE");
    assert_eq!(format!("{}", TokenType::NULL), "NULL");
    assert_eq!(format!("{}", TokenType::STRING_LITERAL), "STRING_LITERAL");
    assert_eq!(format!("{}", TokenType::DECIMAL_LITERAL), "DECIMAL_LITERAL");
    assert_eq!(format!("{}", TokenType::INVALID), "INVALID");
}

// ============================================================================
// ARENA TESTS
// ============================================================================

#[test]
fn test_arena_alloc_token() {
    let mut arena = Arena::new();
    let tok_id = arena.alloc_token(Token::new(
        TokenType::DECIMAL_LITERAL,
        "42".to_string(),
        0,
        2,
    ));
    let tok = arena.get_token(tok_id);
    assert_eq!(tok.image, "42");
    assert_eq!(tok.token_type, TokenType::DECIMAL_LITERAL);
}

#[test]
fn test_arena_alloc_node() {
    let mut arena = Arena::new();
    let tok_id = arena.alloc_token(Token::new(TokenType::EOF, String::new(), 0, 0));
    let node = JmsSelectorNode::new(tok_id, tok_id);
    let node_id = arena.alloc_node(AstNode::JmsSelector(node));
    match arena.get_node(node_id) {
        AstNode::JmsSelector(n) => {
            assert!(n.children.is_empty());
            assert!(n.parent.is_none());
        }
        _ => panic!("Expected JmsSelector"),
    }
}

#[test]
fn test_arena_multiple_nodes_have_distinct_ids() {
    let mut arena = Arena::new();
    let tok_id = arena.alloc_token(Token::new(TokenType::EOF, String::new(), 0, 0));
    let n1 = arena.alloc_node(AstNode::JmsSelector(JmsSelectorNode::new(tok_id, tok_id)));
    let n2 = arena.alloc_node(AstNode::OrExpression(OrExpressionNode::new(tok_id, tok_id)));
    let n3 = arena.alloc_node(AstNode::AndExpression(AndExpressionNode::new(tok_id, tok_id)));
    assert_ne!(n1, n2);
    assert_ne!(n2, n3);
    assert_ne!(n1, n3);
}

#[test]
fn test_arena_get_node_mut() {
    let mut arena = Arena::new();
    let tok_id = arena.alloc_token(Token::new(TokenType::EOF, String::new(), 0, 0));
    let parent_id = arena.alloc_node(AstNode::JmsSelector(JmsSelectorNode::new(tok_id, tok_id)));
    let child_id = arena.alloc_node(AstNode::OrExpression(OrExpressionNode::new(tok_id, tok_id)));

    // Mutably add child and set parent
    match arena.get_node_mut(parent_id) {
        AstNode::JmsSelector(node) => node.children.push(child_id),
        _ => panic!("Expected JmsSelector"),
    }
    match arena.get_node_mut(child_id) {
        AstNode::OrExpression(node) => node.parent = Some(parent_id),
        _ => panic!("Expected OrExpression"),
    }

    // Verify
    match arena.get_node(parent_id) {
        AstNode::JmsSelector(node) => {
            assert_eq!(node.children.len(), 1);
            assert_eq!(node.children[0], child_id);
        }
        _ => panic!("Expected JmsSelector"),
    }
    match arena.get_node(child_id) {
        AstNode::OrExpression(node) => assert_eq!(node.parent, Some(parent_id)),
        _ => panic!("Expected OrExpression"),
    }
}

#[test]
fn test_all_node_types_constructible() {
    let mut arena = Arena::new();
    let tok = arena.alloc_token(Token::new(TokenType::EOF, String::new(), 0, 0));

    // Verify all 12 node types can be allocated
    arena.alloc_node(AstNode::JmsSelector(JmsSelectorNode::new(tok, tok)));
    arena.alloc_node(AstNode::OrExpression(OrExpressionNode::new(tok, tok)));
    arena.alloc_node(AstNode::AndExpression(AndExpressionNode::new(tok, tok)));
    arena.alloc_node(AstNode::EqualityExpression(EqualityExpressionNode::new(tok, tok)));
    arena.alloc_node(AstNode::ComparisonExpression(ComparisonExpressionNode::new(tok, tok)));
    arena.alloc_node(AstNode::AddExpression(AddExpressionNode::new(tok, tok)));
    arena.alloc_node(AstNode::MultExpr(MultExprNode::new(tok, tok)));
    arena.alloc_node(AstNode::UnaryExpr(UnaryExprNode::new(tok, tok)));
    arena.alloc_node(AstNode::PrimaryExpr(PrimaryExprNode::new(tok, tok)));
    arena.alloc_node(AstNode::Literal(LiteralNode::new(tok, tok)));
    arena.alloc_node(AstNode::StringLitteral(StringLitteralNode::new(tok, tok)));
    arena.alloc_node(AstNode::Variable(VariableNode::new(tok, tok)));
    // All 12 node types constructed successfully
}

#[test]
fn test_parser_provides_arena_access() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();
    match parser.arena().get_node(root) {
        AstNode::JmsSelector(_) => { /* success */ }
        _ => panic!("Expected JmsSelector at root"),
    }
}

// ============================================================================
// PRETTY PRINT TESTS
// ============================================================================

#[test]
fn test_pretty_print_shows_input() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0, parser.input());
    assert!(
        output.contains("AST: \"42\""),
        "Pretty print should show original input, got: {}",
        output
    );
}

#[test]
fn test_pretty_print_shows_node_type() {
    // With a simple input "42", passthrough optimization skips single-child wrappers
    // like JmsSelector. Check that a leaf node type is present instead.
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0, parser.input());
    assert!(
        output.contains("Literal"),
        "Pretty print should contain Literal node type, got: {}",
        output
    );
}

#[test]
fn test_pretty_print_shows_token_value() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0, parser.input());
    // The root is a leaf (no children), so it should print the token value
    assert!(
        output.contains("42"),
        "Pretty print should show token value, got: {}",
        output
    );
}

// ============================================================================
// VISITOR TESTS
// ============================================================================

#[test]
fn test_visitor_visits_root() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();

    let mut visited = false;
    parser.arena().visit(
        root,
        &mut |_id, _node, _arena, _depth, _opts| {
            visited = true;
            VisitControl::Continue
        },
        None,
    );
    assert!(visited, "Visitor should visit at least the root node");
}

#[test]
fn test_visitor_root_depth_is_zero() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();

    let mut root_depth = None;
    parser.arena().visit(
        root,
        &mut |_id, _node, _arena, depth, _opts| {
            if root_depth.is_none() {
                root_depth = Some(depth);
            }
            VisitControl::Continue
        },
        None,
    );
    assert_eq!(root_depth, Some(0), "Root should be at depth 0");
}

#[test]
fn test_visitor_skip_children() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();

    let mut count = 0;
    parser.arena().visit(
        root,
        &mut |_id, _node, _arena, _depth, _opts| {
            count += 1;
            VisitControl::SkipChildren
        },
        None,
    );
    assert_eq!(count, 1, "SkipChildren on root should visit only root");
}

#[test]
fn test_visitor_stop() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();

    let mut count = 0;
    parser.arena().visit(
        root,
        &mut |_id, _node, _arena, _depth, _opts| {
            count += 1;
            VisitControl::Stop
        },
        None,
    );
    assert_eq!(count, 1, "Stop should halt traversal immediately");
}

#[test]
fn test_visitor_with_options() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();

    let label = String::from("test_context");
    let mut received = false;
    parser.arena().visit(
        root,
        &mut |_id, _node, _arena, _depth, opts: Option<&dyn Any>| {
            if let Some(val) = opts.and_then(|o| o.downcast_ref::<String>()) {
                if val == "test_context" {
                    received = true;
                }
            }
            VisitControl::Continue
        },
        Some(&label),
    );
    assert!(received, "Visitor should receive the options parameter");
}

#[test]
fn test_visitor_without_options() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();

    let mut saw_none = false;
    parser.arena().visit(
        root,
        &mut |_id, _node, _arena, _depth, opts: Option<&dyn Any>| {
            if opts.is_none() {
                saw_none = true;
            }
            VisitControl::Continue
        },
        None,
    );
    assert!(saw_none, "Options should be None when not provided");
}

#[test]
fn test_visitor_receives_correct_node_id() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    let root = parser.parse().unwrap();

    let mut first_visited_id = None;
    parser.arena().visit(
        root,
        &mut |id, _node, _arena, _depth, _opts| {
            if first_visited_id.is_none() {
                first_visited_id = Some(id);
            }
            VisitControl::Continue
        },
        None,
    );
    assert_eq!(first_visited_id, Some(root), "Visitor should receive the root node ID first");
}

// ============================================================================
// ERROR TYPE TESTS
// ============================================================================

#[test]
fn test_parse_error_new() {
    let err = ParseError::new("test error");
    assert_eq!(err.message, "test error");
    assert!(err.position.is_none());
    assert!(err.line.is_none());
    assert!(err.column.is_none());
}

#[test]
fn test_parse_error_at_position() {
    let err = ParseError::at_position("test", 42);
    assert_eq!(err.position, Some(42));
    assert!(err.line.is_none());
}

#[test]
fn test_parse_error_at_location() {
    let err = ParseError::at_location("test", 3, 7);
    assert_eq!(err.line, Some(3));
    assert_eq!(err.column, Some(7));
    assert!(err.position.is_none());
}

#[test]
fn test_parse_error_with_position() {
    let err = ParseError::new("test").with_position(10);
    assert_eq!(err.position, Some(10));
}

#[test]
fn test_parse_error_with_location() {
    let err = ParseError::new("test").with_location(3, 7);
    assert_eq!(err.line, Some(3));
    assert_eq!(err.column, Some(7));
}

#[test]
fn test_parse_error_display_with_location() {
    let err = ParseError::at_location("test error", 1, 5);
    let msg = format!("{}", err);
    assert!(msg.contains("line 1"), "Should show line: {}", msg);
    assert!(msg.contains("column 5"), "Should show column: {}", msg);
    assert!(msg.contains("test error"), "Should show message: {}", msg);
}

#[test]
fn test_parse_error_display_with_position() {
    let err = ParseError::at_position("test error", 42);
    let msg = format!("{}", err);
    assert!(msg.contains("position 42"), "Should show position: {}", msg);
    assert!(msg.contains("test error"), "Should show message: {}", msg);
}

#[test]
fn test_parse_error_display_plain() {
    let err = ParseError::new("test error");
    let msg = format!("{}", err);
    assert!(msg.contains("test error"), "Should show message: {}", msg);
}

#[test]
fn test_parse_error_is_std_error() {
    let err = ParseError::new("test");
    let _: &dyn std::error::Error = &err; // Verify it implements std::error::Error
}
