//! Lexer implementation for SQL expression parser

use crate::error::{ParseError, ParseResult};
use crate::tokens::{Token, TokenType, LexicalState, TokenSource};

/// The lexer/tokenizer for SQL expressions
pub struct Lexer {
    /// The input string being tokenized
    input: String,
    /// Current position in the input
    position: usize,
    /// Current lexical state
    #[allow(dead_code)]
    state: LexicalState,
    /// Current line number (1-indexed)
    current_line: usize,
    /// Current column number (1-indexed)
    current_column: usize,
    /// Line start offsets for quick line/column lookup
    line_starts: Vec<usize>,
}

impl Lexer {
    /// Create a new lexer for the given input
    pub fn new(input: String) -> Self {
        let mut line_starts = vec![0];
        for (i, ch) in input.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }

        Lexer {
            input,
            position: 0,
            state: LexicalState::DEFAULT,
            current_line: 1,
            current_column: 1,
            line_starts,
        }
    }

    /// Get the next token from the input
    pub fn next_token(&mut self) -> ParseResult<Token> {
        // Skip whitespace
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(Token::new(
                TokenType::EOF,
                String::new(),
                self.position,
                self.position,
            ));
        }

        let start_pos = self.position;
        let start_line = self.current_line;
        let start_column = self.current_column;

        // Try to match each token type
        if let Some(token) = self.try_match_token(start_pos)? {
            return Ok(token);
        }

        // If no token matched, it's an error
        Err(ParseError::at_location(
            format!("Unexpected character: '{}'", self.current_char()),
            start_line,
            start_column,
        ))
    }

    /// Try to match a token at the current position
    fn try_match_token(&mut self, start_pos: usize) -> ParseResult<Option<Token>> {
        let ch = self.current_char();

        // Check for multi-character operators first
        if self.matches_string("<=") {
            self.advance();
            self.advance();
            return Ok(Some(Token::new(TokenType::LE, "<=".to_string(), start_pos, self.position)));
        }
        if self.matches_string(">=") {
            self.advance();
            self.advance();
            return Ok(Some(Token::new(TokenType::GE, ">=".to_string(), start_pos, self.position)));
        }
        if self.matches_string("<>") {
            self.advance();
            self.advance();
            return Ok(Some(Token::new(TokenType::NE, "<>".to_string(), start_pos, self.position)));
        }
        if self.matches_string("!=") {
            self.advance();
            self.advance();
            return Ok(Some(Token::new(TokenType::NE, "!=".to_string(), start_pos, self.position)));
        }

        // Single character operators
        match ch {
            '=' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::EQ, "=".to_string(), start_pos, self.position)));
            }
            '<' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::LT, "<".to_string(), start_pos, self.position)));
            }
            '>' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::GT, ">".to_string(), start_pos, self.position)));
            }
            '+' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::PLUS, "+".to_string(), start_pos, self.position)));
            }
            '-' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::MINUS, "-".to_string(), start_pos, self.position)));
            }
            '*' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::STAR, "*".to_string(), start_pos, self.position)));
            }
            '/' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::SLASH, "/".to_string(), start_pos, self.position)));
            }
            '(' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::LPAREN, "(".to_string(), start_pos, self.position)));
            }
            ')' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::RPAREN, ")".to_string(), start_pos, self.position)));
            }
            ',' => {
                self.advance();
                return Ok(Some(Token::new(TokenType::COMMA, ",".to_string(), start_pos, self.position)));
            }
            _ => {}
        }

        // String literal
        if ch == '\'' {
            return self.match_string_literal(start_pos);
        }

        // Number (integer or decimal)
        if ch.is_ascii_digit() {
            return self.match_number(start_pos);
        }

        // Identifier or keyword
        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.match_identifier_or_keyword(start_pos);
        }

        Ok(None)
    }

    /// Match a string literal (single quotes)
    fn match_string_literal(&mut self, start_pos: usize) -> ParseResult<Option<Token>> {
        self.advance(); // consume opening quote
        let mut image = String::from("'");

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch == '\'' {
                image.push(ch);
                self.advance();
                return Ok(Some(Token::new(
                    TokenType::STRING_LITERAL,
                    image,
                    start_pos,
                    self.position,
                )));
            }
            image.push(ch);
            self.advance();
        }

        Err(ParseError::at_position(
            "Unterminated string literal".to_string(),
            start_pos,
        ))
    }

    /// Match a number (integer or decimal)
    fn match_number(&mut self, start_pos: usize) -> ParseResult<Option<Token>> {
        let mut image = String::new();
        let mut is_decimal = false;

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_ascii_digit() {
                image.push(ch);
                self.advance();
            } else if ch == '.' && !is_decimal {
                // Check if this is a decimal point
                if let Some(next) = self.peek(1) {
                    if next.is_ascii_digit() {
                        is_decimal = true;
                        image.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let token_type = if is_decimal {
            TokenType::DECIMAL_LITERAL
        } else {
            TokenType::INTEGER_LITERAL
        };

        Ok(Some(Token::new(token_type, image, start_pos, self.position)))
    }

    /// Match an identifier or keyword
    fn match_identifier_or_keyword(&mut self, start_pos: usize) -> ParseResult<Option<Token>> {
        let mut image = String::new();

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_ascii_alphanumeric() || ch == '_' {
                image.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for keywords (case insensitive)
        let upper = image.to_uppercase();
        let token_type = match upper.as_str() {
            "AND" => TokenType::AND,
            "OR" => TokenType::OR,
            "NOT" => TokenType::NOT,
            "TRUE" => TokenType::TRUE,
            "FALSE" => TokenType::FALSE,
            "NULL" => TokenType::NULL,
            "LIKE" => TokenType::LIKE,
            "IN" => TokenType::IN,
            "BETWEEN" => TokenType::BETWEEN,
            "IS" => TokenType::IS,
            _ => TokenType::IDENTIFIER,
        };

        Ok(Some(Token::new(token_type, image, start_pos, self.position)))
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Get the current character without consuming it
    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap_or('\0')
    }

    /// Advance to the next character
    fn advance(&mut self) {
        if self.position < self.input.len() {
            let ch = self.current_char();
            self.position += ch.len_utf8();

            if ch == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            } else {
                self.current_column += 1;
            }
        }
    }

    /// Peek ahead n characters without consuming
    fn peek(&self, n: usize) -> Option<char> {
        self.input[self.position..].chars().nth(n)
    }

    /// Check if current position matches a string
    fn matches_string(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }
}

impl TokenSource for Lexer {
    fn get_line_from_offset(&self, offset: usize) -> usize {
        match self.line_starts.binary_search(&offset) {
            Ok(line) => line + 1,
            Err(line) => line,
        }
    }

    fn get_column_from_offset(&self, offset: usize) -> usize {
        let line_num = self.get_line_from_offset(offset);
        if line_num == 0 || line_num > self.line_starts.len() {
            return 1;
        }

        let line_start = self.line_starts[line_num - 1];
        offset.saturating_sub(line_start) + 1
    }
}
