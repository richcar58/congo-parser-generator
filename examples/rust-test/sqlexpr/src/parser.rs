//! Parser implementation for SQL expressions

use crate::arena::{Arena, AstNode, NodeId, TokenId, ComparisonOp};
use crate::arena::{
    SqlExpressionNode, OrExpressionNode, AndExpressionNode, NotExpressionNode,
    ComparisonExpressionNode, ValueListNode, AdditiveExpressionNode,
    MultiplicativeExpressionNode, UnaryExpressionNode, PrimaryExpressionNode,
};
use crate::error::{ParseError, ParseResult};
use crate::tokens::{Token, TokenType};
use crate::lexer::Lexer;

/// The parser for SQL expressions with arena-based AST allocation
pub struct Parser {
    /// The lexer providing tokens
    lexer: Lexer,
    /// Current token being examined
    current_token: Token,
    /// Lookahead tokens
    lookahead: Vec<Token>,
    /// Arena for allocating AST nodes
    arena: Arena,
    /// Current token ID
    current_token_id: Option<TokenId>,
}

impl Parser {
    /// Create a new parser for the given input
    pub fn new(input: String) -> ParseResult<Self> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;

        Ok(Parser {
            lexer,
            current_token,
            lookahead: Vec::new(),
            arena: Arena::new(),
            current_token_id: None,
        })
    }

    /// Get a reference to the arena
    pub fn arena(&self) -> &Arena {
        &self.arena
    }

    /// Get a mutable reference to the arena
    pub fn arena_mut(&mut self) -> &mut Arena {
        &mut self.arena
    }

    /// Parse the input and return the root node ID
    pub fn parse(&mut self) -> ParseResult<NodeId> {
        self.parse_sql_expression()
    }

    /// Parse: SqlExpression -> OrExpression <EOF>
    fn parse_sql_expression(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();
        let child = self.parse_or_expression()?;
        self.expect_token(TokenType::EOF)?;
        let end_token = self.current_token_id.unwrap_or(begin_token);

        let mut node = SqlExpressionNode::new(begin_token, end_token);
        node.children.push(child);

        let node_id = self.arena.alloc_node(AstNode::SqlExpression(node));
        self.set_parent(child, node_id);
        Ok(node_id)
    }

    /// Parse: OrExpression -> AndExpression (<OR> AndExpression)*
    fn parse_or_expression(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();
        let mut children = Vec::new();

        let first = self.parse_and_expression()?;
        children.push(first);

        while self.current_token.token_type == TokenType::OR {
            self.consume_token()?;
            let child = self.parse_and_expression()?;
            children.push(child);
        }

        let end_token = self.current_token_id.unwrap_or(begin_token);
        let mut node = OrExpressionNode::new(begin_token, end_token);
        node.children = children.clone();

        let node_id = self.arena.alloc_node(AstNode::OrExpression(node));
        for child in children {
            self.set_parent(child, node_id);
        }
        Ok(node_id)
    }

    /// Parse: AndExpression -> NotExpression (<AND> NotExpression)*
    fn parse_and_expression(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();
        let mut children = Vec::new();

        let first = self.parse_not_expression()?;
        children.push(first);

        while self.current_token.token_type == TokenType::AND {
            self.consume_token()?;
            let child = self.parse_not_expression()?;
            children.push(child);
        }

        let end_token = self.current_token_id.unwrap_or(begin_token);
        let mut node = AndExpressionNode::new(begin_token, end_token);
        node.children = children.clone();

        let node_id = self.arena.alloc_node(AstNode::AndExpression(node));
        for child in children {
            self.set_parent(child, node_id);
        }
        Ok(node_id)
    }

    /// Parse: NotExpression -> <NOT> NotExpression | ComparisonExpression
    fn parse_not_expression(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();
        let mut is_negated = false;

        if self.current_token.token_type == TokenType::NOT {
            is_negated = true;
            self.consume_token()?;
            let child = self.parse_not_expression()?;
            let end_token = self.current_token_id.unwrap_or(begin_token);

            let mut node = NotExpressionNode::new(begin_token, end_token);
            node.is_negated = is_negated;
            node.children.push(child);

            let node_id = self.arena.alloc_node(AstNode::NotExpression(node));
            self.set_parent(child, node_id);
            return Ok(node_id);
        }

        let child = self.parse_comparison_expression()?;
        let end_token = self.current_token_id.unwrap_or(begin_token);

        let mut node = NotExpressionNode::new(begin_token, end_token);
        node.is_negated = is_negated;
        node.children.push(child);

        let node_id = self.arena.alloc_node(AstNode::NotExpression(node));
        self.set_parent(child, node_id);
        Ok(node_id)
    }

    /// Parse: ComparisonExpression -> AdditiveExpression (comparison_suffix)?
    fn parse_comparison_expression(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();
        let mut children = Vec::new();
        let mut comparison_op = None;

        let left = self.parse_additive_expression()?;
        children.push(left);

        // Check for comparison operators
        match self.current_token.token_type {
            TokenType::EQ => {
                comparison_op = Some(ComparisonOp::Eq);
                self.consume_token()?;
                let right = self.parse_additive_expression()?;
                children.push(right);
            }
            TokenType::NE => {
                comparison_op = Some(ComparisonOp::Ne);
                self.consume_token()?;
                let right = self.parse_additive_expression()?;
                children.push(right);
            }
            TokenType::LT => {
                comparison_op = Some(ComparisonOp::Lt);
                self.consume_token()?;
                let right = self.parse_additive_expression()?;
                children.push(right);
            }
            TokenType::GT => {
                comparison_op = Some(ComparisonOp::Gt);
                self.consume_token()?;
                let right = self.parse_additive_expression()?;
                children.push(right);
            }
            TokenType::LE => {
                comparison_op = Some(ComparisonOp::Le);
                self.consume_token()?;
                let right = self.parse_additive_expression()?;
                children.push(right);
            }
            TokenType::GE => {
                comparison_op = Some(ComparisonOp::Ge);
                self.consume_token()?;
                let right = self.parse_additive_expression()?;
                children.push(right);
            }
            TokenType::LIKE => {
                comparison_op = Some(ComparisonOp::Like);
                self.consume_token()?;
                // LIKE requires a string literal
                self.expect_token(TokenType::STRING_LITERAL)?;
            }
            TokenType::IN => {
                comparison_op = Some(ComparisonOp::In);
                self.consume_token()?;
                self.expect_token(TokenType::LPAREN)?;
                let value_list = self.parse_value_list()?;
                children.push(value_list);
                self.expect_token(TokenType::RPAREN)?;
            }
            TokenType::BETWEEN => {
                comparison_op = Some(ComparisonOp::Between);
                self.consume_token()?;
                let lower = self.parse_additive_expression()?;
                children.push(lower);
                self.expect_token(TokenType::AND)?;
                let upper = self.parse_additive_expression()?;
                children.push(upper);
            }
            TokenType::IS => {
                self.consume_token()?;
                if self.current_token.token_type == TokenType::NOT {
                    self.consume_token()?;
                    self.expect_token(TokenType::NULL)?;
                    comparison_op = Some(ComparisonOp::IsNotNull);
                } else {
                    self.expect_token(TokenType::NULL)?;
                    comparison_op = Some(ComparisonOp::IsNull);
                }
            }
            _ => {}
        }

        let end_token = self.current_token_id.unwrap_or(begin_token);
        let mut node = ComparisonExpressionNode::new(begin_token, end_token);
        node.comparison_op = comparison_op;
        node.children = children.clone();

        let node_id = self.arena.alloc_node(AstNode::ComparisonExpression(node));
        for child in children {
            self.set_parent(child, node_id);
        }
        Ok(node_id)
    }

    /// Parse: ValueList -> AdditiveExpression ("," AdditiveExpression)*
    fn parse_value_list(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();
        let mut children = Vec::new();

        let first = self.parse_additive_expression()?;
        children.push(first);

        while self.current_token.token_type == TokenType::COMMA {
            self.consume_token()?;
            let child = self.parse_additive_expression()?;
            children.push(child);
        }

        let end_token = self.current_token_id.unwrap_or(begin_token);
        let mut node = ValueListNode::new(begin_token, end_token);
        node.children = children.clone();

        let node_id = self.arena.alloc_node(AstNode::ValueList(node));
        for child in children {
            self.set_parent(child, node_id);
        }
        Ok(node_id)
    }

    /// Parse: AdditiveExpression -> MultiplicativeExpression (("+"|"-") MultiplicativeExpression)*
    fn parse_additive_expression(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();
        let mut children = Vec::new();

        let first = self.parse_multiplicative_expression()?;
        children.push(first);

        while self.current_token.token_type == TokenType::PLUS
            || self.current_token.token_type == TokenType::MINUS
        {
            self.consume_token()?;
            let child = self.parse_multiplicative_expression()?;
            children.push(child);
        }

        let end_token = self.current_token_id.unwrap_or(begin_token);
        let mut node = AdditiveExpressionNode::new(begin_token, end_token);
        node.children = children.clone();

        let node_id = self.arena.alloc_node(AstNode::AdditiveExpression(node));
        for child in children {
            self.set_parent(child, node_id);
        }
        Ok(node_id)
    }

    /// Parse: MultiplicativeExpression -> UnaryExpression (("*"|"/") UnaryExpression)*
    fn parse_multiplicative_expression(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();
        let mut children = Vec::new();

        let first = self.parse_unary_expression()?;
        children.push(first);

        while self.current_token.token_type == TokenType::STAR
            || self.current_token.token_type == TokenType::SLASH
        {
            self.consume_token()?;
            let child = self.parse_unary_expression()?;
            children.push(child);
        }

        let end_token = self.current_token_id.unwrap_or(begin_token);
        let mut node = MultiplicativeExpressionNode::new(begin_token, end_token);
        node.children = children.clone();

        let node_id = self.arena.alloc_node(AstNode::MultiplicativeExpression(node));
        for child in children {
            self.set_parent(child, node_id);
        }
        Ok(node_id)
    }

    /// Parse: UnaryExpression -> "-" UnaryExpression | PrimaryExpression
    fn parse_unary_expression(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();

        if self.current_token.token_type == TokenType::MINUS {
            self.consume_token()?;
            let child = self.parse_unary_expression()?;
            let end_token = self.current_token_id.unwrap_or(begin_token);

            let mut node = UnaryExpressionNode::new(begin_token, end_token);
            node.is_negated = true;
            node.children.push(child);

            let node_id = self.arena.alloc_node(AstNode::UnaryExpression(node));
            self.set_parent(child, node_id);
            return Ok(node_id);
        }

        let child = self.parse_primary_expression()?;
        let end_token = self.current_token_id.unwrap_or(begin_token);

        let mut node = UnaryExpressionNode::new(begin_token, end_token);
        node.children.push(child);

        let node_id = self.arena.alloc_node(AstNode::UnaryExpression(node));
        self.set_parent(child, node_id);
        Ok(node_id)
    }

    /// Parse: PrimaryExpression -> literal | identifier | "(" OrExpression ")"
    fn parse_primary_expression(&mut self) -> ParseResult<NodeId> {
        let begin_token = self.alloc_current_token();
        let mut children = Vec::new();

        match self.current_token.token_type {
            TokenType::INTEGER_LITERAL
            | TokenType::DECIMAL_LITERAL
            | TokenType::STRING_LITERAL
            | TokenType::TRUE
            | TokenType::FALSE
            | TokenType::NULL
            | TokenType::IDENTIFIER => {
                self.consume_token()?;
            }
            TokenType::LPAREN => {
                self.consume_token()?;
                let inner = self.parse_or_expression()?;
                children.push(inner);
                self.expect_token(TokenType::RPAREN)?;
            }
            _ => {
                return Err(ParseError::at_position(
                    format!(
                        "Expected expression, found {:?} '{}'",
                        self.current_token.token_type, self.current_token.image
                    ),
                    self.current_token.begin_offset,
                ));
            }
        }

        let end_token = self.current_token_id.unwrap_or(begin_token);
        let mut node = PrimaryExpressionNode::new(begin_token, end_token);
        node.children = children.clone();

        let node_id = self.arena.alloc_node(AstNode::PrimaryExpression(node));
        for child in children {
            self.set_parent(child, node_id);
        }
        Ok(node_id)
    }

    // ========== Helper Methods ==========

    /// Allocate the current token in the arena and return its ID
    fn alloc_current_token(&mut self) -> TokenId {
        let token_id = self.arena.alloc_token(self.current_token.clone());
        self.current_token_id = Some(token_id);
        token_id
    }

    /// Set the parent of a node
    fn set_parent(&mut self, child_id: NodeId, parent_id: NodeId) {
        match self.arena.get_node_mut(child_id) {
            AstNode::SqlExpression(node) => node.parent = Some(parent_id),
            AstNode::OrExpression(node) => node.parent = Some(parent_id),
            AstNode::AndExpression(node) => node.parent = Some(parent_id),
            AstNode::NotExpression(node) => node.parent = Some(parent_id),
            AstNode::ComparisonExpression(node) => node.parent = Some(parent_id),
            AstNode::ValueList(node) => node.parent = Some(parent_id),
            AstNode::AdditiveExpression(node) => node.parent = Some(parent_id),
            AstNode::MultiplicativeExpression(node) => node.parent = Some(parent_id),
            AstNode::UnaryExpression(node) => node.parent = Some(parent_id),
            AstNode::PrimaryExpression(node) => node.parent = Some(parent_id),
        }
    }

    /// Consume the current token and advance to the next one
    fn consume_token(&mut self) -> ParseResult<Token> {
        let old_token = self.current_token.clone();
        self.current_token = if !self.lookahead.is_empty() {
            self.lookahead.remove(0)
        } else {
            self.lexer.next_token()?
        };
        self.current_token_id = Some(self.arena.alloc_token(self.current_token.clone()));
        Ok(old_token)
    }

    /// Expect a specific token type and consume it
    fn expect_token(&mut self, expected: TokenType) -> ParseResult<Token> {
        if self.current_token.token_type == expected {
            self.consume_token()
        } else {
            Err(ParseError::at_position(
                format!(
                    "Expected {:?}, found {:?} '{}'",
                    expected, self.current_token.token_type, self.current_token.image
                ),
                self.current_token.begin_offset,
            ))
        }
    }
}
