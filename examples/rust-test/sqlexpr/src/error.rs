//! Error types for SQL expression parser

use std::fmt;

/// Result type for parsing operations
pub type ParseResult<T> = Result<T, ParseError>;

/// A parsing error with location information
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The error message
    pub message: String,
    /// Optional position in the input
    pub position: Option<usize>,
    /// Optional line number (1-indexed)
    pub line: Option<usize>,
    /// Optional column number (1-indexed)
    pub column: Option<usize>,
}

impl ParseError {
    /// Create a new error with just a message
    pub fn new(message: impl Into<String>) -> Self {
        ParseError {
            message: message.into(),
            position: None,
            line: None,
            column: None,
        }
    }

    /// Create an error with position information
    pub fn at_position(message: impl Into<String>, position: usize) -> Self {
        ParseError {
            message: message.into(),
            position: Some(position),
            line: None,
            column: None,
        }
    }

    /// Create an error with line/column information
    pub fn at_location(message: impl Into<String>, line: usize, column: usize) -> Self {
        ParseError {
            message: message.into(),
            position: None,
            line: Some(line),
            column: Some(column),
        }
    }

    /// Add position information
    pub fn with_position(mut self, position: usize) -> Self {
        self.position = Some(position);
        self
    }

    /// Add line/column information
    pub fn with_location(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(pos) = self.position {
            write!(f, " at position {}", pos)?;
        }
        if let (Some(line), Some(col)) = (self.line, self.column) {
            write!(f, " at line {}, column {}", line, col)?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {}
