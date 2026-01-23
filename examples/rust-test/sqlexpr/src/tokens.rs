//! Token types for SQL expression parser

use crate::arena::TokenId;

/// All token types in the SQL expression grammar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Keywords (case insensitive)
    AND,
    OR,
    NOT,
    TRUE,
    FALSE,
    NULL,
    LIKE,
    IN,
    BETWEEN,
    IS,

    // Comparison operators
    EQ,      // =
    NE,      // <> or !=
    LT,      // <
    GT,      // >
    LE,      // <=
    GE,      // >=

    // Arithmetic operators
    PLUS,    // +
    MINUS,   // -
    STAR,    // *
    SLASH,   // /

    // Delimiters
    LPAREN,  // (
    RPAREN,  // )
    COMMA,   // ,

    // Literals
    INTEGER_LITERAL,
    DECIMAL_LITERAL,
    STRING_LITERAL,

    // Identifier
    IDENTIFIER,

    // End of file
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::AND => write!(f, "AND"),
            TokenType::OR => write!(f, "OR"),
            TokenType::NOT => write!(f, "NOT"),
            TokenType::TRUE => write!(f, "TRUE"),
            TokenType::FALSE => write!(f, "FALSE"),
            TokenType::NULL => write!(f, "NULL"),
            TokenType::LIKE => write!(f, "LIKE"),
            TokenType::IN => write!(f, "IN"),
            TokenType::BETWEEN => write!(f, "BETWEEN"),
            TokenType::IS => write!(f, "IS"),
            TokenType::EQ => write!(f, "="),
            TokenType::NE => write!(f, "<>"),
            TokenType::LT => write!(f, "<"),
            TokenType::GT => write!(f, ">"),
            TokenType::LE => write!(f, "<="),
            TokenType::GE => write!(f, ">="),
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            TokenType::STAR => write!(f, "*"),
            TokenType::SLASH => write!(f, "/"),
            TokenType::LPAREN => write!(f, "("),
            TokenType::RPAREN => write!(f, ")"),
            TokenType::COMMA => write!(f, ","),
            TokenType::INTEGER_LITERAL => write!(f, "INTEGER"),
            TokenType::DECIMAL_LITERAL => write!(f, "DECIMAL"),
            TokenType::STRING_LITERAL => write!(f, "STRING"),
            TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

/// Lexical state for context-sensitive tokenization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LexicalState {
    #[default]
    DEFAULT,
}

/// A single token
#[derive(Debug, Clone)]
pub struct Token {
    /// The type of this token
    pub token_type: TokenType,
    /// The literal text of this token
    pub image: String,
    /// Start offset in the input
    pub begin_offset: usize,
    /// End offset in the input (exclusive)
    pub end_offset: usize,
    /// Link to next token (for token chaining)
    pub next: Option<TokenId>,
    /// Link to previous token (for token chaining)
    pub previous: Option<TokenId>,
}

impl Token {
    /// Create a new token
    pub fn new(token_type: TokenType, image: String, begin_offset: usize, end_offset: usize) -> Self {
        Token {
            token_type,
            image,
            begin_offset,
            end_offset,
            next: None,
            previous: None,
        }
    }

    /// Get the length of this token in characters
    pub fn len(&self) -> usize {
        self.image.len()
    }

    /// Check if this token is empty
    pub fn is_empty(&self) -> bool {
        self.image.is_empty()
    }

    /// Check if this token is of a specific type
    pub fn is_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }

    /// Check if this token is one of the given types
    pub fn is_one_of(&self, types: &[TokenType]) -> bool {
        types.contains(&self.token_type)
    }
}

/// Trait for getting position information from a token source
pub trait TokenSource {
    /// Get the line number for an offset (1-indexed)
    fn get_line_from_offset(&self, offset: usize) -> usize;
    /// Get the column number for an offset (1-indexed)
    fn get_column_from_offset(&self, offset: usize) -> usize;
}
