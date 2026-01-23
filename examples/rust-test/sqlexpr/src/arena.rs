//! Arena allocator for SQL expression AST nodes

use crate::tokens::Token;

/// Type-safe index for nodes in the arena
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// Type-safe index for tokens in the arena
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenId(pub usize);

/// Arena that owns all AST nodes and tokens
pub struct Arena {
    /// All AST nodes
    nodes: Vec<AstNode>,
    /// All tokens
    tokens: Vec<Token>,
}

impl Arena {
    /// Create a new empty arena
    pub fn new() -> Self {
        Arena {
            nodes: Vec::new(),
            tokens: Vec::new(),
        }
    }

    /// Allocate a new node in the arena
    pub fn alloc_node(&mut self, node: AstNode) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(node);
        id
    }

    /// Get a reference to a node
    pub fn get_node(&self, id: NodeId) -> &AstNode {
        &self.nodes[id.0]
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, id: NodeId) -> &mut AstNode {
        &mut self.nodes[id.0]
    }

    /// Allocate a new token in the arena
    pub fn alloc_token(&mut self, token: Token) -> TokenId {
        let id = TokenId(self.tokens.len());
        self.tokens.push(token);
        id
    }

    /// Get a reference to a token
    pub fn get_token(&self, id: TokenId) -> &Token {
        &self.tokens[id.0]
    }

    /// Pretty print the AST starting from the given node
    pub fn pretty_print(&self, root: NodeId, indent: usize) -> String {
        let mut result = String::new();
        self.pretty_print_impl(root, indent, &mut result);
        result
    }

    fn pretty_print_impl(&self, node_id: NodeId, indent: usize, result: &mut String) {
        let indent_str = "  ".repeat(indent);
        match self.get_node(node_id) {
            AstNode::SqlExpression(node) => {
                result.push_str(&format!("{}SqlExpression\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::OrExpression(node) => {
                result.push_str(&format!("{}OrExpression\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::AndExpression(node) => {
                result.push_str(&format!("{}AndExpression\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::NotExpression(node) => {
                result.push_str(&format!("{}NotExpression\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::ComparisonExpression(node) => {
                result.push_str(&format!("{}ComparisonExpression\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::ValueList(node) => {
                result.push_str(&format!("{}ValueList\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::AdditiveExpression(node) => {
                result.push_str(&format!("{}AdditiveExpression\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::MultiplicativeExpression(node) => {
                result.push_str(&format!("{}MultiplicativeExpression\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::UnaryExpression(node) => {
                result.push_str(&format!("{}UnaryExpression\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::PrimaryExpression(node) => {
                let token = self.get_token(node.begin_token);
                result.push_str(&format!("{}PrimaryExpression(\"{}\")\n", indent_str, token.image));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
        }
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

/// Enum containing all AST node types
#[derive(Debug, Clone)]
pub enum AstNode {
    SqlExpression(SqlExpressionNode),
    OrExpression(OrExpressionNode),
    AndExpression(AndExpressionNode),
    NotExpression(NotExpressionNode),
    ComparisonExpression(ComparisonExpressionNode),
    ValueList(ValueListNode),
    AdditiveExpression(AdditiveExpressionNode),
    MultiplicativeExpression(MultiplicativeExpressionNode),
    UnaryExpression(UnaryExpressionNode),
    PrimaryExpression(PrimaryExpressionNode),
}

/// AST node for SqlExpression production
#[derive(Debug, Clone)]
pub struct SqlExpressionNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
}

impl SqlExpressionNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        SqlExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// AST node for OrExpression production
#[derive(Debug, Clone)]
pub struct OrExpressionNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
}

impl OrExpressionNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        OrExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// AST node for AndExpression production
#[derive(Debug, Clone)]
pub struct AndExpressionNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
}

impl AndExpressionNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        AndExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// AST node for NotExpression production
#[derive(Debug, Clone)]
pub struct NotExpressionNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
    pub is_negated: bool,
}

impl NotExpressionNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        NotExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
            is_negated: false,
        }
    }
}

/// AST node for ComparisonExpression production
#[derive(Debug, Clone)]
pub struct ComparisonExpressionNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
    pub comparison_op: Option<ComparisonOp>,
}

/// Comparison operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Like,
    In,
    Between,
    IsNull,
    IsNotNull,
}

impl ComparisonExpressionNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        ComparisonExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
            comparison_op: None,
        }
    }
}

/// AST node for ValueList production
#[derive(Debug, Clone)]
pub struct ValueListNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
}

impl ValueListNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        ValueListNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// AST node for AdditiveExpression production
#[derive(Debug, Clone)]
pub struct AdditiveExpressionNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
}

impl AdditiveExpressionNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        AdditiveExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// AST node for MultiplicativeExpression production
#[derive(Debug, Clone)]
pub struct MultiplicativeExpressionNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
}

impl MultiplicativeExpressionNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        MultiplicativeExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// AST node for UnaryExpression production
#[derive(Debug, Clone)]
pub struct UnaryExpressionNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
    pub is_negated: bool,
}

impl UnaryExpressionNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        UnaryExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
            is_negated: false,
        }
    }
}

/// AST node for PrimaryExpression production
#[derive(Debug, Clone)]
pub struct PrimaryExpressionNode {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub begin_token: TokenId,
    pub end_token: TokenId,
}

impl PrimaryExpressionNode {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        PrimaryExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}
