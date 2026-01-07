//! Integration tests for arithmetic parser

use arithmetic::*;

#[test]
fn test_simple_number() {
    let mut parser = Parser::new("42".to_string()).unwrap();
    assert!(parser.parse().is_ok());
}

#[test]
fn test_addition() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    assert!(parser.parse().is_ok());
}

#[test]
fn test_complex_expression() {
    let mut parser = Parser::new("(1 + 2) * 3 - 4 / 5".to_string()).unwrap();
    assert!(parser.parse().is_ok());
}

#[test]
fn test_nested_parentheses() {
    let mut parser = Parser::new("((1 + 2) * (3 - 4))".to_string()).unwrap();
    assert!(parser.parse().is_ok());
}

#[test]
fn test_invalid_syntax_missing_operand() {
    let mut parser = Parser::new("1 +".to_string()).unwrap();
    assert!(parser.parse().is_err());
}

#[test]
fn test_invalid_syntax_missing_paren() {
    let mut parser = Parser::new("(1 + 2".to_string()).unwrap();
    assert!(parser.parse().is_err());
}

#[test]
fn test_invalid_start() {
    let mut parser = Parser::new("* 1".to_string()).unwrap();
    assert!(parser.parse().is_err());
}

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
fn test_error_messages_include_position() {
    let mut parser = Parser::new("* 1".to_string()).unwrap();
    let err = parser.parse().unwrap_err();
    let err_str = format!("{}", err);

    // Error message should include position information
    assert!(err_str.contains("position"));
}
